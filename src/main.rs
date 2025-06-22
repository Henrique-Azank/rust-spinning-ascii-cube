
/*
    Main entrypoint for the spinning cube program.
*/
fn main() {
    
    // Instantiate some variables for the program
    let cube_width: f64 = 10.0;

    // Backdrgound ASCII code
    const BACKGROUND_CHAR: char = ' ';

    // Window dimensions
    const WIDTH: usize = 160;
    const HEIGHT: usize = 44;

    // Instantiate an array buffer 
    let buffer: [f64; WIDTH * HEIGHT] = [0.0; WIDTH * HEIGHT];

    // Instantiate an ASCII character buffer
    let char_buffer: [char; WIDTH * HEIGHT] = [BACKGROUND_CHAR; WIDTH * HEIGHT];

    // Instantiate an increment speed for the cube
    let increment_speed: f64 = 0.1;

    // Main running loop
    loop {
        // Instantiate a cube X
        let mut cube_x: f64 = -cube_width;

        // For loop through a cube X
        while cube_x < cube_width {

            // Instantiate a cube Y
            let mut cube_y: f64 = -cube_width;

            // For loop through a cube Y
            while cube_y < cube_width {

                // Increment the cube Y
                cube_y += increment_speed;
            }

            // Increment the cube X
            cube_x += increment_speed;
        }
    }
}
