/// Read entries from an *.osrm file.

use byteorder::{ByteOrder, LittleEndian};
use std::io::Read;
use tar;


/// Size of a node in bytes.
const NODE_SIZE: usize = 16;

/// Size of an edge in bytes.
const EDGE_SIZE: usize = 32;


/// A reader for *.osrm files that allows iterating over it's entries.
pub struct OsrmReader<R: Read> {
    archive: tar::Archive<R>,
}

impl<R: Read> OsrmReader<R> {
    /// Creates a new `OsrmReader`.
    pub fn new(reader: R) -> OsrmReader<R> {
        OsrmReader {
            archive: tar::Archive::new(reader),
        }
    }

    /// Returns iterator of entries.
    pub fn entries(&mut self) -> Result<OsrmEntries<R>, std::io::Error> {
        let entries = self.archive.entries()?;
        Ok(OsrmEntries {
            entries,
        })
    }
}

/// An iterator over entries.
///
/// An `Entry` can contain nodes, edges or other content.
pub struct OsrmEntries<'a, R: 'a + Read> {
    entries: tar::Entries<'a, R>,
}

pub enum Entry<'a, R: Read> {
    Nodes(OsrmNodes<'a, R>),
    Edges(OsrmEdges<'a, R>),
    Unknown(tar::Entry<'a, R>),
}

impl<'a, R: 'a + Read> Iterator for OsrmEntries<'a, R> {
    type Item = Result<Entry<'a, R>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.entries.next().map(|result| {
            return match result {
                Ok(entry) => {
                    let path = entry.header().path()?;
                    let path = path.to_str();
                    match path {
                        Some("/extractor/nodes") => Ok(Entry::Nodes(OsrmNodes::new(entry)?)),
                        Some("/extractor/edges") => Ok(Entry::Edges(OsrmEdges::new(entry)?)),
                        Some(_) => Ok(Entry::Unknown(entry)),
                        None => Ok(Entry::Unknown(entry)),
                    }
                },
                Err(err) => Err(err),
            };
        })
    }
}

/// An iterator over nodes.
pub struct OsrmNodes<'a, R: Read> {
    entry: tar::Entry<'a, R>,
    pub number_of_nodes: u64,
    current_node_index: u64,
}

/// A node, a 2D point with latitude and longitude coordinates.
#[derive(Clone, Debug)]
pub struct Node {
    /// raw_longitude = longitude * 1000000
    pub raw_longitude: i32,
    /// raw_latitude = latitude * 1000000
    pub raw_latitude: i32,
    /// A node ID, usually the OSM node ID of the original node.
    pub node_id: i64,
}

impl Node {
    /// Returns longitude as a decimal value.
    pub fn longitude(&self) -> f64 {
        self.raw_longitude as f64 * 0.000001
    }

    /// Returns latitude as a decimal value.
    pub fn latitude(&self) -> f64 {
        self.raw_latitude as f64 * 0.000001
    }
}

impl<'a, R: 'a + Read> OsrmNodes<'a, R> {
    fn new(entry: tar::Entry<'a, R>) -> Result<OsrmNodes<'a, R>, std::io::Error> {
        let size = entry.header().size()?;
        let number_of_nodes = size / NODE_SIZE as u64;

        if size % NODE_SIZE as u64 != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Node entry size is not divisible by the size of a node.")
            );
        }

        Ok(OsrmNodes {
            entry,
            number_of_nodes,
            current_node_index: 0,
        })
    }
}

impl<'a, R: 'a + Read> Iterator for OsrmNodes<'a, R> {
    type Item = Result<Node, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; NODE_SIZE];

        if self.current_node_index >= self.number_of_nodes {
            // Already read last node
            return None;
        }

        if let Err(err) = self.entry.read_exact(&mut buf) {
            return Some(Err(err));
        }
        self.current_node_index += 1;

        let raw_longitude = LittleEndian::read_i32(&buf[0..4]);
        let raw_latitude = LittleEndian::read_i32(&buf[4..8]);
        let node_id = LittleEndian::read_i64(&buf[8..16]);

        Some(Ok(Node{
            raw_longitude,
            raw_latitude,
            node_id,
        }))
    }
}

/// An iterator over edges.
pub struct OsrmEdges<'a, R: Read> {
    entry: tar::Entry<'a, R>,
    pub number_of_edges: u64,
    current_edge_index: u64,
}

/// A connection between two nodes (source and target).
///
/// Nodes are referenced by their index which starts at zero and encodes the order in which the
/// nodes are stored. The node index should not be confused with the node ID.
#[derive(Clone, Debug)]
pub struct Edge {
    /// Index of the first point of the edge.
    pub source_node_index: u32,
    /// Index of the second point of the edge.
    pub target_node_index: u32,
}

impl<'a, R: 'a + Read> OsrmEdges<'a, R> {
    fn new(entry: tar::Entry<'a, R>) -> Result<OsrmEdges<'a, R>, std::io::Error> {
        let size = entry.header().size()?;
        let number_of_edges = size / EDGE_SIZE as u64;

        if size % EDGE_SIZE as u64 != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Edge entry size is not divisible by the size of an edge.")
            );
        }

        Ok(OsrmEdges {
            entry,
            number_of_edges,
            current_edge_index: 0,
        })
    }
}

impl<'a, R: 'a + Read> Iterator for OsrmEdges<'a, R> {
    type Item = Result<Edge, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; EDGE_SIZE];

        if self.current_edge_index >= self.number_of_edges {
            // Already read last edge
            return None;
        }

        if let Err(err) = self.entry.read_exact(&mut buf) {
            return Some(Err(err));
        }
        self.current_edge_index += 1;

        let source_node_index = LittleEndian::read_u32(&buf[0..4]);
        let target_node_index = LittleEndian::read_u32(&buf[4..8]);

        Some(Ok(Edge{
            source_node_index,
            target_node_index,
        }))
    }
}
