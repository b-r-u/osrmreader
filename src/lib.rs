/*!
A fast reader for the `*.osrm` file format.

These files are used by the routing engine OSRM and are usually extracted from
OpenStreetMap data with the tool `osrm-extract`. An `*.osrm` file encodes the
routing graph as nodes and edges.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
osrmreader = "0.1"
```

## Example

Here's a simple example that prints all nodes and edges:

```rust
use osrmreader::{OsrmReader, Entry};

fn main() -> Result<(), std::io::Error> {
    let f = std::fs::File::open("tests/test.osrm")?;
    let mut reader = OsrmReader::new(f);

    for entry in reader.entries()? {
        match entry {
            Ok(Entry::Nodes(nodes)) => {
                // Read nodes
                for n in nodes {
                    println!("{:?}", n?);
                }
            },
            Ok(Entry::Edges(edges)) => {
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
```
*/

pub use osrm::*;

pub mod osrm;
