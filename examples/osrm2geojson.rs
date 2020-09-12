/// Convert a given *.osrm file to GeoJSON.

use osrmreader::{OsrmReader, Entry};
use std::io::{BufWriter, Write};
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    // Command line arguments
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 || !args[1].ends_with(".osrm") || !args[2].ends_with(".geojson") {
        println!("Convert *.osrm files to GeoJSON.\nUsage:\n{} <input.osrm> <output.geojson>", args[0]);
        return Ok(());
    }

    let input_path = &args[1];
    let output_path = &args[2];

    // Open files for reading/writing
    let mut reader = OsrmReader::new(File::open(input_path)?);
    let mut output = BufWriter::new(File::create(output_path)?);

    // Store coordinates of nodes. Nodes are referenced by their index.
    let mut nodes_vec = vec![];

    output.write(b"{\"type\": \"FeatureCollection\", \"features\": [")?;

    let mut edges_written = 0;

    for entry in reader.entries()? {
        match entry? {
            Entry::Nodes(nodes) => {
                // Reserve memory for the expected number of nodes.
                nodes_vec.reserve(nodes.number_of_nodes as usize);

                // Read nodes
                for n in nodes {
                    let n = n?;
                    // Store raw coordinates as i32 to save some space
                    nodes_vec.push((n.raw_latitude, n.raw_longitude))
                }
            },
            Entry::Edges(edges) => {
                // Read edges
                let mut first = true;
                for e in edges {
                    if !first {
                        write!(output, ",")?;
                    }
                    first = false;

                    let e = e?;

                    // Look up nodes
                    let source = nodes_vec[e.source_node_index as usize];
                    let target = nodes_vec[e.target_node_index as usize];

                    // Convert raw coordinates and keep full precision for writing the line string
                    write!(
                        output,
                        "\n{{\"type\": \"Feature\", \"geometry\": {{\"type\": \"LineString\", \"coordinates\": [[{:.6}, {:.6}], [{:.6}, {:.6}]]}}}}",
                        source.1 as f64 * 0.000001,
                        source.0 as f64 * 0.000001,
                        target.1 as f64 * 0.000001,
                        target.0 as f64 * 0.000001,
                    )?;
                    edges_written += 1;
                }
            },
            _ => {},
        }
    }

    // Properly end the GeoJSON file
    output.write(b"\n]}")?;
    output.flush()?;

    println!("Done. Wrote {} edges to {}", edges_written, output_path);
    Ok(())
}
