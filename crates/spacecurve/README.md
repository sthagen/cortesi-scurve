spacecurve
==========

A Rust library for N-dimensional space-filling curves and spatial indexing.

## Features

*   **Supported Curves:**
    *   **Hilbert** (2D optimized, N-D generic)
    *   **Z-order / Morton** (optimized bit-interleaving)
    *   **Gray Code** (Binary Reflected)
    *   **H-curve**
    *   **Scan** (Boustrophedon)
    *   **Onion** / **Hairy Onion** (Recursive layer-based)
*   **High Performance:** Uses `SmallVec` to avoid heap allocations for common 2D/3D points, and optimized SWAR algorithms for bit manipulation.
*   **Generic:** Supports N-dimensional mappings where applicable.

## Usage

```rust
use std::error::Error;

use spacecurve::{curve_from_name, SpaceCurve};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a 2D Hilbert curve of order 3 (8x8 grid)
    let curve = curve_from_name("hilbert", 2, 8)?;

    let point = curve.point(10);
    println!("Point at index 10: {:?}", point);

    let index = curve.index(&point);
    assert_eq!(index, 10);

    Ok(())
}
```

More usage is available in `examples/hilbert.rs`.
