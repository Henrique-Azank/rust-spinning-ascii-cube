
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
    const DISTANCE_FROM_CAMERA: f64 = 150.0;

    // Backdrgound ASCII code
    const BACKGROUND_CHAR: char = ' ';

    // Instantiate a horizontal offset
    const HORIZONTAL_OFFSET: f64 = CUBE_WIDTH / 2.0 as f64;

    // Instantiate an increment speed for the cube
    const INCREMENT_SPEED: f64 = 0.6;

    // Window dimensions
    const WIDTH: usize = 60;
    const HEIGHT: usize = 30;

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

        // print!("\x1b[H");
        print!("\x1b[2J");  

        // Clear the buffer
        for i in 0..(WIDTH * HEIGHT) {
            buffer[i] = 0.0;
            char_buffer[i] = BACKGROUND_CHAR;
        }

        // Instantiate a cube X
        let mut cube_x: f64 = -CUBE_WIDTH;

        // For loop through a cube X
        while cube_x < CUBE_WIDTH {

            // Instantiate a cube Y
            let mut cube_y: f64 = -CUBE_WIDTH;

            // For loop through a cube Y
            while cube_y < CUBE_WIDTH {

                // Create the cube surfaces 
                let cube_surfaces: [(f64, f64, f64, char); 6] = [
                    (cube_x, cube_y, -CUBE_WIDTH, '@'), // Front face
                    (CUBE_WIDTH, cube_y, cube_x, '!'), // Back face
                    (-CUBE_WIDTH, cube_y, -cube_x, '-'), // Top face
                    (-cube_x, cube_y, CUBE_WIDTH, '*'), // Bottom face
                    (cube_x, -CUBE_WIDTH, -cube_y, '?'), // Left face
                    (cube_x, CUBE_WIDTH, cube_y, '0'), // Right face
                ];

                for &(cube_x, cube_y, cube_z, surface_char) in &cube_surfaces {
                    // Get the cube surface coordinate
                    let coordinates: (f64, f64, f64) = (cube_x, cube_y, cube_z);
                    // Calculate the surface rotation
                    let new_coords: (f64, f64, f64) = calculate_surface(
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
                        surface_char,
                    );
                }

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
        rotations.0 += 0.1;
        rotations.1 += 0.1;
        rotations.2 += 0.1;

        // Sleep for a short duration to control the speed of the animation
        std::thread::sleep(std::time::Duration::from_millis(100));

    }
}
