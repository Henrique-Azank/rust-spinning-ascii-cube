
// Use the calculate_rotations module
use crate::calculations::calculate_rotations::{
    calculate_x,
    calculate_y,
    calculate_z,
};

/*
    Function to calculate a surface for the cube
    based on the axis and the cube width.
*/
pub fn calculate_surface(
    // Axis of the cube
    cube: (f64, f64, f64),
    distance_from_camera: f64,
) -> (f64, f64, f64) {

    // Calculate the x component
    let x: f64 = calculate_x(cube);

    // Calculate the y component
    let y: f64 = calculate_y(cube);

    // Calculate the z component (adds a distance from the camera)
    let z: f64 = calculate_z(cube) + distance_from_camera;                 

    // Finish the calculation by returning Ok
    (x, y, z)
}

/*
    Function to calculate surface parameters
*/
pub fn calculate_chars(
    // Get the coordinates
    coordinates: (f64, f64, f64),
) {

    // Calculate the inverse of the z value
    let ooz: f64 = 1.0 / coordinates.2;

    // Get the integer values of xp and yp   


}




