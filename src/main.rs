
// Use the project's modules
use rust_spinning_ascii_cube::calculations::calculate_surface::{
    calculate_surface,
    calculate_chars,
};
use rust_spinning_ascii_cube::console::prints::print_buffer;

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

    // Instantiate a k1 constant factor
    const K1: f64 = 40.0;

    // Mutable loop variables ----------

    // Instantiate an array buffer 
    let mut buffer: [f64; WIDTH * HEIGHT] = [0.0; WIDTH * HEIGHT];

    // Instantiate an ASCII character buffer
    let mut char_buffer: [char; WIDTH * HEIGHT] = [BACKGROUND_CHAR; WIDTH * HEIGHT];
    
    // Instantiate the rotation angles
    let mut rotations: (f64, f64, f64) = (0.0, 0.0, 0.0);

    // Main running loop
    loop {

        // Clear screen + reset cursor
        print!("\x1b[2J\x1b[H");  

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
                let coordinates: (f64, f64, f64) = (cube_x, cube_y, cube_z);

                // Calculate the surface rotation
                let new_coords = calculate_surface(
                    coordinates,
                    rotations,
                    DISTANCE_FROM_CAMERA,
                );

                // Calculate the chars of the surface
                calculate_chars(
                    new_coords,
                    WIDTH,
                    HEIGHT,
                    K1,
                    HORIZONTAL_OFFSET,
                    &mut buffer,
                    &mut char_buffer,
                    '0',
                );

                // Increment the cube Y
                cube_y += INCREMENT_SPEED;
            }

            // Increment the cube X
            cube_x += INCREMENT_SPEED;
        }

        // Print the buffer to the console
        if let Err(e) = print_buffer(&char_buffer, WIDTH, HEIGHT) {
            eprintln!("Error printing buffer: {}", e);
        }

        // Increment the rotation angles
        rotations.0 += 0.05;
        rotations.1 += 0.05;
        rotations.2 += 0.01;
    }
}
