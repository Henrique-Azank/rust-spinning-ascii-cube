
// Instantiate some constants for the re-projection
static A: f64 = 0.0;
static B: f64 = 0.0;
static C: f64 = 0.0;

/*
    Calculate the rotated coordinates of the 3D point
    in the X axis.
*/
pub fn calculate_x(i: f64, j: f64, k: f64) -> f64 {

    // Use the static values to calculate the X component
    let x: f64 = j * A.sin() * B.sin() * C.cos()
                -k * A.cos() * B.sin() * C.cos()
                +j * A.cos() * C.sin()
                +k * A.sin() * C.sin()
                +i * B.cos() * C.cos();

    // Return the X component of the projected vector
    return x;
}

/*
    Calculate the rotated coordinates of the 3D point
    in the Y axis.
*/
pub fn calculate_y(i: f64, j: f64, k: f64) -> f64 {

    // Use the static values to calculate the Y component
    let y: f64 = j * A.cos() * C.cos()
                +k * A.sin() * C.cos()
                -j * A.sin() * B.sin() * C.sin()
                +k * A.cos() * B.sin() * C.sin()
                -i * B.cos() * C.sin();

    // Return the Y component of the projected vector
    return y;
}

/*
    Calculate the rotated coordinates of the 3D point
    in the Z axis.
*/
pub fn calculate_z(i: f64, j: f64, k: f64) -> f64 {

    // Use the static values to calculate the Y component
    let z: f64 = k * A.cos() * B.cos()
                -j * A.sin() * B.cos()
                +i * B.sin();

    // Return the Y component of the projected vector
    return z;
}





