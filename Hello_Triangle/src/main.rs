extern crate minifb;
use std::vec;
use std::path::Path;

use Hello_Triangle::geometry::Mesh;
use glam::Vec3Swizzles;
use glam::{Vec2, Vec3, Vec3A, Vec4};
use minifb::{Key, Window, WindowOptions};

use Hello_Triangle::*;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;


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
    
    let window_size = glam::vec2(WIDTH as f32, HEIGHT as f32);

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

    let mut triangles: [Triangle; 2] = [Triangle::default(); 2];

    triangles[0] = Triangle {
        vert0: vertex0,
        vert1: vertex1,
        vert2: vertex2,
    };

    triangles[1] = Triangle {
        vert0: vertex0,
        vert1: vertex2,
        vert2: vertex3,
    };

    //println!("interpolated verted: {:?}", Lerp(triangles[0].vert0, Lerp(triangles[0].vert1, 0.5)); //explodes
    let mut trianglesGood = vec![glam::uvec3(0,1,2), glam::uvec3(0,2,3)];
    let mut verticesGood = vec![vertex0, vertex1, vertex2, vertex3];

    let mut mesh = Mesh::new();
    mesh.add_section_from_vertices(&mut trianglesGood, &mut verticesGood);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        //may need a screen clear
        clear_buffer(&mut buffer, 0);
        clear_buffer(&mut z_buffer, f32::INFINITY);

        if window.is_key_down(Key::Space) {
            redTriangle = true;
        } else if window.is_key_down(Key::D) {
            wireFrameRend = true;
        } else {
            redTriangle = false;
            wireFrameRend = false;
        }
        
        if redTriangle 
        {
            Funky_Triangle(triangles[0], &mut buffer);
            Funky_Triangle(triangles[1], &mut buffer);
        } else if wireFrameRend {
            //Render_Depth(triangles[0], &mut buffer, &mut z_buffer);
            //Render_Depth(triangles[1], &mut buffer, &mut z_buffer);
            println!("Doesnt work");
        } else {
            //Raster_Triangle(triangles[0], &mut buffer, &texture, &mut z_buffer);
            //Raster_Triangle(triangles[1], &mut buffer, &texture, &mut z_buffer);
            
            

            raster_mesh(&mesh, &texture, &mut buffer, &mut z_buffer, window_size);
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
