
/*
    Calculate the rotated coordinates of the 3D point
    in the X axis.
*/
pub fn calculate_x(
    coordinates: (f64, f64, f64),
    rotations: (f64, f64, f64)
) -> f64 {
    // Get the vector values from the coordiates
    let (i, j, k) = coordinates;

    // Get the rotation values
    let (a, b, c) = rotations;

    // Use the static values to calculate the X component
    let x: f64 = j * a.sin() * b.sin() * c.cos()
                -k * a.cos() * b.sin() * c.cos()
                +j * a.cos() * c.sin()
                +k * a.sin() * c.sin()
                +i * b.cos() * c.cos();

    // Return the X component of the projected vector
    return x;
}

/*
    Calculate the rotated coordinates of the 3D point
    in the Y axis.
*/
pub fn calculate_y(
    coordinates: (f64, f64, f64),
    rotations: (f64, f64, f64)
) -> f64 {

    // Get the vector values from the coordiates
    let (i, j, k) = coordinates;

    // Get the rotation values
    let (a, b, c) = rotations;

    // Use the static values to calculate the Y component
    let y: f64 = j * a.cos() * c.cos()
                +k * a.sin() * c.cos()
                -j * a.sin() * b.sin() * c.sin()
                +k * a.cos() * b.sin() * c.sin()
                -i * b.cos() * c.sin();

    // Return the Y component of the projected vector
    return y;
}

/*
    Calculate the rotated coordinates of the 3D point
    in the Z axis.
*/
pub fn calculate_z(
    coordinates: (f64, f64, f64),
    rotations: (f64, f64, f64)
) -> f64 {

    // Get the vector values from the coordiates
    let (i, j, k) = coordinates;

    // Get the rotation values
    let (a, b, _c) = rotations;

    // Use the static values to calculate the Y component
    let z: f64 = k * a.cos() * b.cos()
                -j * a.sin() * b.cos()
                +i * b.sin();

    // Return the Y component of the projected vector
    return z;
}





