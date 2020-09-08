/*!
A fast reader for the *.osrm file format.

These files are produced by the OSRM-tool `osrm-extract` and encode the routing graph that is
usually extracted from OpenStreetMap data.


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
    let f = std::fs::File::open("tests/map.osrm")?;
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
