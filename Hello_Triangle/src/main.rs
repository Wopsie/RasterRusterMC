extern crate minifb;
use glam::Vec3Swizzles;
use glam::{Vec2, Vec3, Vec3A, Vec4};
use minifb::{Key, Window, WindowOptions};

pub mod utils;
pub use utils::*;
pub mod geometry;
pub use geometry::Vertex;
pub use geometry::Triangle;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

pub fn Raster_Triangle(tri: Triangle, buffer: &mut Vec<u32>)
{
    for (i, pixel) in buffer.iter_mut().enumerate() 
    {
        let coords = index_to_coords(i, HEIGHT);
        //shadowing a variable
        let coords = glam::vec2(coords.0 as f32, coords.1 as f32);
        let area = edge_function(
            tri.vert0.position.xy(), 
            tri.vert1.position.xy(), 
            tri.vert2.position.xy(),
        );

        //if let Some(bary) = Barycentric_Coordinates()
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![to_argb8(255, 255, 0, 0); WIDTH * HEIGHT];

    let mut redTriangle: bool = true;
    let mut wireFrameRend: bool = false;

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let triangle = [
        glam::vec2(100.0, 100.0),
        glam::vec2(250.0, 400.0),
        glam::vec2(400.0, 100.0),
    ];

    let vertex0 = Vertex{
        position: glam::vec3(100.0, 100.0, 0.0),
        color: glam::vec3(1.0, 0.0, 0.0),
        texCoord: glam::vec2(1.0, 1.0),
    };
    let vertex1 = Vertex{
        position: glam::vec3(250.0, 400.0, 0.0),
        color: glam::vec3(0.0, 1.0, 0.0),
        texCoord: glam::vec2(1.0, 1.0),
    };
    let vertex2 = Vertex {
        position: glam::vec3(400.0, 100.0, 0.0),
        color: glam::vec3(0.0, 0.0, 1.0),
        texCoord: glam::vec2(1.0, 1.0),
    };

    let triangle1 = Triangle {
        vert0: vertex0,
        vert1: vertex1,
        vert2: vertex2,
    };

    let mut count: usize = 0;
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::Space) {
            redTriangle = true;
        } else if window.is_key_down(Key::W) {
            wireFrameRend = true;
        } else {
            redTriangle = false;
            wireFrameRend = false;
        }


        if wireFrameRend
        {
        //     // do bresenham here

        //     // buffer is linear u32 array of size width * height
        //     // should find screen space coords of vertices, connect the dots

        //     let vert0 = triangle[0];
        //     let vert1 = triangle[1];
        //     let vert2 = triangle[2];

        //     //write to buffer
        //     let deltax = vert1.x - vert0.x;
        //     let deltay = vert1.y - vert0.y;
        //     let error = deltax * 0.5;
        //     let ystep = 1;
        //     continue;

        //     pass mutable reference to bresenham function? WIP
        }

        for i in buffer.iter_mut() {
            *i = 0; //screen clear

            //screen space coords
            let coords = index_to_coords(count, HEIGHT);
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32);
            count += 1;

            let mut m0 : f32 = 0.0;
            let mut m1 : f32 = 0.0;
            let mut m2 : f32 = 0.0;
            
            if !wireFrameRend
            {
                m0 = edge_function(triangle1.vert0.position.xy(), triangle1.vert1.position.xy(), coords);
                m1 = edge_function(triangle1.vert1.position.xy(), triangle1.vert2.position.xy(), coords);
                m2 = edge_function(triangle1.vert2.position.xy(), triangle1.vert0.position.xy(), coords);
            } else {
                //do bresenham here
                // m0 = bresenham_function(triangle[0], triangle[1], coords);
                // m1 = bresenham_function(triangle[1], triangle[2], coords);
                // m2 = bresenham_function(triangle[2], triangle[0], coords);
                //become the lines, not the edge, does not get filled
            }

            if !redTriangle {
                *i = to_argb8(
                    255,
                    (m2 * 255.0) as u8,
                    (m0 * 255.0) as u8,
                    (m1 * 255.0) as u8,
                );
            } else if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
                *i = to_argb8(255, 255, 0, 0);
            }
        }
        count = 0;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    // for i in 0..buffer.len()
    // {
    //     let buffVal = buffer[i] / 255;
    //     dbg!("Debugging m values: {0: <10}", buffVal);
    //     //dbg!("Debugging m values: {0: <10} | {0: <10} | {0: <10}", m0, m1, m2);
    // }

}
