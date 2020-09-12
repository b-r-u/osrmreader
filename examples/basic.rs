/// Print all nodes and edges of an *.osrm file.

use osrmreader::{OsrmReader, Entry};

fn main() -> Result<(), std::io::Error> {
    let f = std::fs::File::open("tests/test.osrm")?;
    let mut reader = OsrmReader::new(f);

    for entry in reader.entries()? {
        match entry? {
            Entry::Nodes(nodes) => {
                // Read nodes
                for n in nodes {
                    println!("{:?}", n?);
                }
            },
            Entry::Edges(edges) => {
                // Read edges
                for e in edges {
                    println!("{:?}", e?);
                }
            },
            _ => {},
        }
    }

    Ok(())
}
