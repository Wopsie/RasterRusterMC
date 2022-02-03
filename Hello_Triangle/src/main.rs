extern crate minifb;

//use glam::{Vec3, Vec3A, Vec4};
use glam::{Vec2};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

pub fn edge_function_wc(v0: Vec2, v1: Vec2, p: Vec2) -> f32
{
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub fn index_to_coords(p: usize, height: usize) -> (usize,usize) {
    (p % height, p / height)
}

//convert colors to u32
pub fn to_argb8(a: u8, r:u8, g:u8, b:u8) -> u32
{
    let mut argb: u32 = a as u32; //a
    argb = (argb << 8) + r as u32; //r
    argb = (argb << 8) + g as u32; //g
    argb = (argb << 8) + b as u32; //b
    argb 
    //you can do "shadowing" on the data(?)
    //by default statics cannot be mutable
    //same for constants
        //need to use "unsafe" for that
    
}

fn main() {
    let mut buffer: Vec<u32> = vec![to_argb8(255, 255, 255, 255); WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let edge = (glam::vec2(0.0,0.0), Vec2::new(WIDTH as f32, HEIGHT as f32));

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    for i in 0..buffer.len() {
        
        let coords = index_to_coords(i, HEIGHT);
        let coords = glam::vec2(coords.0 as f32, coords.1 as f32);
        let side = edge_function_wc(edge.0, edge.1, coords);

        if side >= 0.0 {
            buffer  [i] = to_argb8(255, 0, 255, 0);
        }else {
            buffer  [i] = to_argb8(255, 255, 0, 0);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {

        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}