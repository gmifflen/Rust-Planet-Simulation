use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;

// width and height, duh
const WIDTH: usize = 800;
const HEIGHT: usize = 800;

// astronomical unit in meters (average distance from Earth to the Sun)
const AU: f64 = 149.6e6 * 1000.0;

// gravitational constant in the SI unit of m^3 kg^-1 s^-2
const G: f64 = 6.67428e-11;

// used to avoid numerical instability in gravitational calculations at close distances
const SOFTENING_FACTOR: f64 = 1.0e9;

// scaling factor to convert astronomical units to screen coordinates
const SCALE: f64 = 250.0 / AU;

// time step for the simulation in seconds (1 day in this case)
const TIMESTEP: f64 = 3600.0 * 24.0;

#[derive(Clone, Debug)]
struct Planet {
    x: f64,                 // x-coordinate of the planet's position
    y: f64,                 // y-coordinate of the planet's position
    radius: f64,            // radius of the planet
    color: u32,             // color code for visualization
    mass: f64,              // mass of the planet
    orbit: Vec<(f64, f64)>, // list of orbital coordinates for visualization
    sun: bool,              // indicates whether the planet represents the sun
    distance_to_sun: f64,   // distance from the planet to the sun
    x_vel: f64,             // velocity of the planet along the x-axis
    y_vel: f64,             // velocity of the planet along the y-axis
}

impl Planet {
    // create a new planet with the given properties
    const fn new(x: f64, y: f64, radius: f64, color: u32, mass: f64) -> Self {
        Self {
            x,
            y,
            radius,
            color,
            mass,
            orbit: Vec::new(),
            sun: false,
            distance_to_sun: 0.0,
            x_vel: 0.0,
            y_vel: 0.0,
        }
    }

    // draw the planet on the window and update its orbit path
    fn draw(&self, buffer: &mut [u32]) {
        // update the orbit path to visualize the planet's movement
        self.update_orbit_points(buffer);

        // calculate the planet's position on the window and draw it
        let x = self.x.mul_add(SCALE, WIDTH as f64 / 2.0) as usize;
        let y = self.y.mul_add(SCALE, HEIGHT as f64 / 2.0) as usize;
        draw_circle(buffer, x, y, self.radius as usize, self.color);

        // display the distance to the sun (unless it's the sun itself)
        if !self.sun {
            let distance_text = format!("{:.1}km", self.distance_to_sun / 1000.0);
            draw_text(buffer, &distance_text, x, y, self.color);
        }
    }

    // keep the orbit path up to date with the planet's current position
    fn update_orbit_points(&self, buffer: &mut [u32]) {
        // calculate and update the visual orbit path
        let updated_points: Vec<(usize, usize)> = self
            .orbit
            .iter()
            .map(|&(x, y)| {
                (
                    x.mul_add(SCALE, WIDTH as f64 / 2.0) as usize,
                    y.mul_add(SCALE, HEIGHT as f64 / 2.0) as usize,
                )
            })
            .collect();

        // draw lines connecting the updated orbit points
        for window in updated_points.windows(2) {
            draw_line(
                buffer,
                window[0].0,
                window[0].1,
                window[1].0,
                window[1].1,
                self.color,
            );
        }
    }

    // update the planet's position and velocity due to gravitational forces
    fn update_position(&mut self, planets: &[Self]) {
        println!("Before Update: x = {}, y = {}, x_vel = {}, y_vel = {}", self.x, self.y, self.x_vel, self.y_vel);
        // initialize variables to store total force components
        let mut total_force_x = 0.0;
        let mut total_force_y = 0.0;

        // calculate gravitational forces between this planet and others
        for other in planets {
            if self as *const _ != other as *const _ {
                let (fx, fy, distance) = self.calculate_force(other);
                if !fx.is_finite() || !fy.is_finite() {
                    eprintln!("Non-finite force calculated for planet: {self:?}");
                    eprintln!("Position of other planet involved: {other:?}");
                    continue;
                }

                // update total force components and track the distance to the sun (if it's the sun)
                total_force_x += fx;
                total_force_y += fy;
                println!("Total Force: fx = {}, fy = {}", total_force_x, total_force_y);

                if other.sun {
                    self.distance_to_sun = distance;
                }
            }
        }

        // update velocities based on the total force and time step
        self.x_vel += total_force_x / self.mass * TIMESTEP;
        self.y_vel += total_force_y / self.mass * TIMESTEP;

        // check for non-finite velocities and skip position update in case of errors
        if !self.x_vel.is_finite() || !self.y_vel.is_finite() {
            eprintln!(
                "Non-finite velocity calculated: x_vel = {}, y_vel = {}",
                self.x_vel, self.y_vel
            );
            return; // skip updating the position to avoid further issues
        }

        // calculate and update new positions based on updated velocities
        let new_x = self.x_vel.mul_add(TIMESTEP, self.x);
        let new_y = self.y_vel.mul_add(TIMESTEP, self.y);

        // check if the updated positions are finite
        if new_x.is_finite() && new_y.is_finite() {
            self.x = new_x;
            self.y = new_y;

        } else {
            eprintln!("Non-finite position calculated: x = {new_x}, y = {new_y}");
        }


        // add the current position to the orbit path for the visual effect
        self.orbit.push((self.x, self.y));
        println!("After Update: x = {}, y = {}, x_vel = {}, y_vel = {}", self.x, self.y, self.x_vel, self.y_vel);
    }

    // Calculate gravitational force components and distance to another planet
    fn calculate_force(&self, other: &Self) -> (f64, f64, f64) {
        // calculate the components and magnitude of the distance between two planets
        let distance_x = other.x - self.x;
        let distance_y = other.y - self.y;
        let distance_squared = distance_x.powi(2) + distance_y.powi(2) + SOFTENING_FACTOR.powi(2);
        let distance = distance_squared.sqrt();

        // calculate the gravitational force components and direction
        let force = G * self.mass * other.mass / distance.powi(2);
        let theta = distance_y.atan2(distance_x);
        let force_x = theta.cos() * force;
        let force_y = theta.sin() * force;

        // return force components and distance
        (force_x, force_y, distance)
    }
}

// draw a line on the buffer using Bresenham's line drawing algorithm
fn draw_line(buffer: &mut [u32], x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
    // calculate the differences and steps in the x and y directions
    let dx = (x2 as isize - x1 as isize).abs();
    let dy = -(y2 as isize - y1 as isize).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    // initialize the starting point (x, y)
    let mut x = x1 as isize;
    let mut y = y1 as isize;

    // iterate over the points along the line and update the buffer
    while x != x2 as isize || y != y2 as isize {
        if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
            buffer[(y * WIDTH as isize + x) as usize] = color;
        }

        // calculate the next error and move in the appropriate direction
        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x += sx;
        }

        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

// draw a circle on the buffer using the Midpoint Circle Algorithm
fn draw_circle(buffer: &mut [u32], _x: usize, _y: usize, radius: usize, color: u32) {
    // initialize variables for circle drawing
    let mut x = radius as isize - 1;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 1;
    let mut err = dx - (radius << 1) as isize;

    // loop through the points of the circle and update the buffer
    while x >= y {
        // define eight symmetric points based on the current circle position
        let points = [
            (_x as isize + x, _y as isize + y),
            (_x as isize - x, _y as isize + y),
            (_x as isize + x, _y as isize - y),
            (_x as isize - x, _y as isize - y),
            (_x as isize + y, _y as isize + x),
            (_x as isize - y, _y as isize + x),
            (_x as isize + y, _y as isize - x),
            (_x as isize - y, _y as isize - x),
        ];

        // update the buffer with the points if they are within bounds
        for &(px, py) in &points {
            if px >= 0 && px < WIDTH as isize && py >= 0 && py < HEIGHT as isize {
                buffer[(py * WIDTH as isize + px) as usize] = color;
            }
        }

        // update position and error for the next point on the circle
        if err <= 0 {
            y += 1;
            err += dy;
            dy += 2;
        }

        if err > 0 {
            x -= 1;
            dx += 2;
            err += dx - (radius << 1) as isize;
        }
    }
}

// draw text on the buffer using a simple font and specified parameters
fn draw_text(buffer: &mut [u32], text: &str, x: usize, y: usize, color: u32) {
    // calculate the width and height of the text
    let text_width = text.len() * 8;
    let text_height = 8;

    // determine the starting position for drawing the text
    let x_start = if x >= text_width / 2 {
        x - text_width / 2
    } else {
        0
    };
    let y_start = if y >= text_height / 2 {
        y - text_height / 2
    } else {
        0
    };

    // iterate over characters in the text and draw each character's pixels
    for (i, c) in text.chars().enumerate() {
        let mut mask = 0x80;
        for j in 0..8 {
            let pixel_x = x_start + i * 8 + j;
            let pixel_y = y_start;

            // check if the pixel is within the buffer boundaries
            if pixel_x >= WIDTH || pixel_y >= HEIGHT {
                continue;
            }

            // determine the pixel color based on the font and mask
            let pixel_color = if FONT[c as usize * 8 + j] & mask == 0 {
                0
            } else {
                color
            };

            // set the pixel color in the buffer
            buffer[pixel_y * WIDTH + pixel_x] = pixel_color;
            mask >>= 1;
        }
    }
}

// array for storing font data, initialized with default (zero) values
const FONT: [u8; 256 * 8] = [0u8; 256 * 8];
// const FONT: [u8; 256 * 8] = include!("font.bin");

// clear the buffer by setting all pixels to the default color (0)
fn clear_buffer(buffer: &mut [u32]) {
    for pixel in buffer.iter_mut() {
        *pixel = 0;
    }
}

fn main() {
    // initialize the simulation parameters and create a window
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Planet Simulation", WIDTH, HEIGHT, WindowOptions::default())
        .expect("failed to create window. minifb or dependencies problem.");

    // create planet objects and set their initial properties
    let mut sun = Planet::new(0.0, 0.0, 30.0, 0x00FF_FF00, 1.98892_f64 * 10.0_f64.powi(30));
    sun.sun = true;
    let mut earth = Planet::new(
        -1.0 * AU,
        0.0,
        16.0,
        0x0064_95ED,
        5.9742_f64 * 10.0_f64.powi(24),
    );
    earth.y_vel = 29.783 * 1000.0;
    let mut mars = Planet::new(
        -1.524 * AU,
        0.0,
        12.0,
        0x00BC_2732,
        6.39_f64 * 10.0_f64.powi(23),
    );
    mars.y_vel = 24.077 * 1000.0;
    let mut mercury = Planet::new(
        0.387 * AU,
        0.0,
        8.0,
        0x0050_4E51,
        3.30_f64 * 10.0_f64.powi(23),
    );
    mercury.y_vel = -47.4 * 1000.0;
    let mut venus = Planet::new(
        0.723 * AU,
        0.0,
        14.0,
        0x00FF_FFFF,
        4.8685_f64 * 10.0_f64.powi(24),
    );
    venus.y_vel = -35.02 * 1000.0;
    
    let mut planets = vec![sun, earth, mars, mercury, venus];

    // main simulation loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Make a clone of the current state of planets for reading
        let planets_clone = planets.clone();

        // Iterate over planets with indices
        for (i, planet) in planets.iter_mut().enumerate() {
            // Create a slice of all planets except the current one
            let others = [&planets_clone[..i], &planets_clone[i + 1..]].concat();

            // Update the position of the current planet
            planet.update_position(&others);
        }

        // clear the buffer and draw the planets
        clear_buffer(&mut buffer);

        for planet in &planets {
            planet.draw(&mut buffer);
        }

        // update the window with the buffer and handle errors
        if let Err(e) = window.update_with_buffer(&buffer, WIDTH, HEIGHT) {
            eprintln!("error updating window buffer: {e}");
        }

        // sleep to control the simulation frame rate
        thread::sleep(Duration::from_micros(16667));
    }
}
