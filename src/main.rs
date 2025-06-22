
// Use the project's modules
use rust_spinning_ascii_cube::calculations::calculate_surface::{
    calculate_surface,
    calculate_chars,
};

/*
    Main entrypoint for the spinning cube program.
*/
fn main() {

    // Constant parameters ----------
    
    // Instantiate some variables for the program
    const CUBE_WIDTH: f64 = 10.0;

    // Instantiate a distance from camera float
    const DISTANCE_FROM_CAMERA: f64 = 60.0;

    // Backdrgound ASCII code
    const BACKGROUND_CHAR: char = ' ';

    // Instantiate a horizontal offset
    const HORIZONTAL_OFFSET: f64 = - 2.0 * CUBE_WIDTH as f64;

    // Instantiate an increment speed for the cube
    const INCREMENT_SPEED: f64 = 0.1;

    // Window dimensions
    const WIDTH: usize = 160;
    const HEIGHT: usize = 44;

    // Mutable loop variables ----------

    // Instantiate an array buffer 
    let buffer: [f64; WIDTH * HEIGHT] = [0.0; WIDTH * HEIGHT];

    // Instantiate an ASCII character buffer
    let char_buffer: [char; WIDTH * HEIGHT] = [BACKGROUND_CHAR; WIDTH * HEIGHT];

    // Main running loop
    loop {
        // Instantiate a cube X
        let mut cube_x: f64 = -CUBE_WIDTH;

        // For loop through a cube X
        while cube_x < CUBE_WIDTH {

            // Instantiate a cube Y
            let mut cube_y: f64 = -CUBE_WIDTH;

            // For loop through a cube Y
            while cube_y < CUBE_WIDTH {

                // Instantiate a cube Z
                let cube_z: f64 = -CUBE_WIDTH;

                // Get the cube surface coordinate
                let surface: (f64, f64, f64) = (cube_x, cube_y, cube_z);

                // Calculate the surface rotation
                let new_coords = calculate_surface(
                    surface,
                    DISTANCE_FROM_CAMERA,
                );

                // Calculate the chars of the surface

                // Increment the cube Y
                cube_y += INCREMENT_SPEED;
            }

            // Increment the cube X
            cube_x += INCREMENT_SPEED;
        }
    }
}
