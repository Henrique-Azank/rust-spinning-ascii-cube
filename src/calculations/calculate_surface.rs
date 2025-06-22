
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
    coordinates: (f64, f64, f64),
    rotations: (f64, f64, f64),
    distance_from_camera: f64,
) -> (f64, f64, f64) {

    // Calculate the x component
    let x: f64 = calculate_x(
        coordinates,
        rotations
    );

    // Calculate the y component
    let y: f64 = calculate_y(
        coordinates, 
        rotations
    );

    // Calculate the z component (adds a distance from the camera)
    let z: f64 = calculate_z(
        coordinates, 
        rotations,
    ) + distance_from_camera;                 

    // Finish the calculation by returning Ok
    (x, y, z)
}

/*
    Function to calculate surface parameters
*/
pub fn calculate_chars(
    // Get the coordinates
    coordinates: (f64, f64, f64),
    // Get the width and height of the screen
    width: usize,
    height: usize,
    // Get a K1 factor
    k1: f64,
    // Horizontal offset
    horizontal_offset: f64,
    // Buffers to alter
    buffer: &mut [f64],
    char_buffer: &mut [char],
    // Character to use for the surface
    surface_char: char,
) {

    // Calculate the inverse of the z value
    let ooz: f64 = 1.0 / coordinates.2;

    // Get the value of xp
    let xp: usize = (
        (width as f64 / 2.0) + 
        (horizontal_offset + k1 * ooz * coordinates.0 * 2.0) 
    ) as usize;

    // Get the value of yp
    let yp: usize = (
        (height as f64 / 2.0) +
        (k1 * ooz * coordinates.1 * 2.0) 
    ) as usize;

    // Calculate idx
    let idx: usize = (yp * width) + xp;

    // Check the bounds to fill the buffers
    if idx < buffer.len() {

        // If ooz is greater than the buffer value
        if ooz > buffer[idx] {
            // Set the buffer value
            buffer[idx] = ooz;
            // Set the character buffer value
            char_buffer[idx] = surface_char;
        }
    }
}




