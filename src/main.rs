extern crate minifb;

use minifb::{Key, Window, WindowOptions};

mod convert;
pub mod structs;
use structs::vector2::Vector2;
use structs::Raycast::{*};
use structs::Raycast::Raycasters::*;
use structs::WorldObj::*;

const CAMERA_FOV: f32 = 40.;
const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    // Window dimensions


    // Create a buffer to store pixel data
    let mut buffer: Vec<u32>;


    // Create a window
    let mut window = Window::new(
        "Doom-Like",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut world1 = World{
        walls: Vec::new(),
    };
    world1.walls.push( Wall{
        p1: Vector2{x: -10., y:10.},
        p2: Vector2{x: 20., y:25.}
    });
    let ray1 = Ray{
        startPos: Vector2{x: 0., y:0.},
        angle: -1.,
        distance: 20.,
    };
    match shootRay(&ray1,&world1) {
        // The division was valid
        Some(ray) => println!("Result: {}, {}",  ray.point.x, ray.point.y),
        // The division was invalid
        None    => println!("Nothing in raycast range."),
    }
    let (mut x, mut y, mut rotation) = (0.,0.,0.);
    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        buffer = vec![0; WIDTH * HEIGHT];

        let mut cur_pixel = 1;
        while WIDTH+1>cur_pixel {
            rotation = rotation%360.;
            let angle = CAMERA_FOV/(WIDTH as f32) * cur_pixel as f32 + rotation;
            let ray = Ray{
                startPos: Vector2{x: x, y:y},
                angle: angle,
                distance: 300.,
            };
            match shootRay(&ray,&world1) {
                // The ray hit an wall
                Some(ray) => turn_pixel_red_on_horizontal(cur_pixel, &mut buffer, angle, ray.distance),
                // The ray didn't hit anything
                None => ()//println!("Missed at pixel : {} and angle: {}", cur_pixel, angle),
            }
            cur_pixel += 1;
        }
        if window.is_key_down(Key::A) {
            println!("Key A is pressed!");
            x -= 5.;
        }

        if window.is_key_down(Key::D) {
            println!("Key B is pressed!");
            x += 5.;
        }
        if window.is_key_down(Key::S) {
            println!("Key S is pressed!");
            y -= 5.;
        }

        if window.is_key_down(Key::W) {
            println!("Key W is pressed!");
            y += 5.;
        }


        if window.is_key_down(Key::E) {
            println!("Key E is pressed!");
            rotation += 5.;
        }

        if window.is_key_down(Key::Q) {
            println!("Key Q is pressed!");
            rotation -= 5.;
        }
        // Draw the buffer to the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn turn_pixel_red_on_horizontal(width: usize, buffer: &mut Vec<u32>, angle: f32, distance: f32) {
    let dimension = HEIGHT as f32/300. * (300. - distance);
    for i in 0..HEIGHT{

        buffer[width + i * WIDTH - 1] = convert::from_u8_rgb(0, 255, 0);
    }

    println!("Hit at pixel : {} and angle: {} with ray distance: {}!", width, angle, distance)
}