extern crate minifb;
//use core::prelude::v1;
use std::vec;
use std::path::Path;

use glam::Vec3Swizzles;
use glam::{Vec2, Vec3, Vec3A, Vec4};
use minifb::{Key, Window, WindowOptions};

pub mod utils;
pub use utils::*;
pub mod geometry;
pub use geometry::Vertex;
pub use geometry::Triangle;
pub use geometry::Point;
pub mod texture;
pub use texture::Texture;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

pub fn Raster_Triangle(tri: Triangle, buffer: &mut Vec<u32>, texture: &Texture, z_buffer: &mut Vec<f32>)
{
    for (i, pixel) in buffer.iter_mut().enumerate() 
    {
        let coords = index_to_coords(i, HEIGHT);
        //shadowing a variable
        let coords = glam::vec2(coords.0 as f32, coords.1 as f32) + 0.5;
        let area = edge_function(
            tri.vert0.position.xy(), 
            tri.vert1.position.xy(), 
            tri.vert2.position.xy(),
        );

        if let Some(bary) = Barycentric_Coordinates(
            coords,
            tri.vert0.position.xy(),
            tri.vert1.position.xy(),
            tri.vert2.position.xy(),
            area,
        ) {
            // bary var presumably contains barycentric coordinates of the given coords on the given triangle
            let depth = bary.x * tri.vert0.position.z + bary.y * tri.vert1.position.z + bary.z * tri.vert2.position.z;
            if depth < z_buffer[i] {
                z_buffer[i] = depth;
                
                let texCoords = bary.x * tri.vert0.uv + bary.y * tri.vert1.uv + bary.z * tri.vert2.uv;
                let color = texture.argb_at_uv(texCoords.x, texCoords.y);

                *pixel = color; //write to buffer
            }
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Test - Do not press 'W'",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = vec![to_argb8(255, 0, 0, 0); WIDTH * HEIGHT];
    let mut z_buffer = vec![f32::INFINITY; WIDTH * HEIGHT];
    
    let texture = Texture::Load(Path::new("D:/BUAS/MC/Rust/RasterRusterMC/Hello_Triangle/Assets/bojan.jpg"));

    let mut redTriangle: bool = true;
    let mut wireFrameRend: bool = false;


    let vertex0 = Vertex{
        position: glam::vec3(100.0, 100.0, 0.0),
        color: glam::vec3(1.0, 0.0, 0.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let vertex1 = Vertex{
        position: glam::vec3(100.0, 400.0, 0.0),
        color: glam::vec3(0.0, 1.0, 0.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let vertex2 = Vertex {
        position: glam::vec3(400.0, 400.0, 0.0),
        color: glam::vec3(0.0, 0.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
    };
    let vertex3 = Vertex {
        position: glam::vec3(400.0, 100.0, 0.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 0.0),
    };

    let triangle1 = Triangle {
        vert0: vertex0,
        vert1: vertex1,
        vert2: vertex2,
    };

    let triangle2 = Triangle {
        vert0: vertex0,
        vert1: vertex2,
        vert2: vertex3,
    };
    
    //let triang = Triangle::default();

    //let triangles: Vec<Triangle> = vec![Triangle::default(); 2];
    //let triangles: Vec<Triangle> = vec![triangle1; 2];

    //let mut triangles: [Triangle; 1] = [triangle1; 1];

    // triangles[1] = Triangle {
    //     vert0: vertex0,
    //     vert1: vertex2,
    //     vert2: vertex3,
    // };


    //Raster_Triangle(&triangles[0], &mut buffer, &texture, &mut z_buffer);
    Raster_Triangle(triangle1, &mut buffer, &texture, &mut z_buffer);
    Raster_Triangle(triangle2, &mut buffer, &texture, &mut z_buffer);


    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::Space) {
            redTriangle = true;
        } else if window.is_key_down(Key::W) {
            wireFrameRend = true;
        } else {
            redTriangle = false;
            wireFrameRend = false;
        }
        
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
