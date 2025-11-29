//! Minimal Hilbert curve example: map an index to a point and back.

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // snips-start: example
    // 2D Hilbert curve on an 8x8 grid (order 3)
    let curve = spacecurve::curve_from_name("hilbert", 2, 8)?;
    println!(
        "{}D Hilbert length: {} cells",
        curve.dimensions(),
        curve.length()
    );

    let index = 10;
    let point = curve.point(index);
    println!("Point at index {index}: {:?}", point);

    let round_trip = curve.index(&point);
    println!("Index for {:?}: {round_trip}", point);

    assert_eq!(round_trip, index);
    // snips-end: example

    Ok(())
}
