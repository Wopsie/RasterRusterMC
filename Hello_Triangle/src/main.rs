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

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 8.0)),
        frustum_far: 1000.0,
        ..Default::default()
    };

    let texture = Texture::Load(Path::new("D:/BUAS/MC/Rust/RasterRusterMC/Hello_Triangle/Assets/bojan.jpg"));

    let mut redTriangle: bool = true;
    let mut wireFrameRend: bool = false;

    let vertex0 = Vertex{
        position: glam::vec3(-2.0, -2.0, 2.0),
        color: glam::vec3(1.0, 0.0, 0.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let vertex1 = Vertex{
        position: glam::vec3(-2.0, 2.0, 2.0),
        color: glam::vec3(0.0, 1.0, 0.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let vertex2 = Vertex {
        position: glam::vec3(2.0, 2.0, 2.0),
        color: glam::vec3(0.0, 0.0, 1.0),
        uv: glam::vec2(1.0, 0.0),
    };
    let vertex3 = Vertex {
        position: glam::vec3(2.0, -2.0, 2.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
    };

    let vertex4 = Vertex{
        position: glam::vec3(-2.0, -2.0, -2.0),
        color: glam::vec3(1.0, 0.0, 0.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let vertex5 = Vertex{
        position: glam::vec3(-2.0, 2.0, -2.0),
        color: glam::vec3(0.0, 1.0, 0.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let vertex6 = Vertex {
        position: glam::vec3(2.0, 2.0, -2.0),
        color: glam::vec3(0.0, 0.0, 1.0),
        uv: glam::vec2(1.0, 0.0),
    };
    let vertex7 = Vertex {
        position: glam::vec3(2.0, -2.0, -2.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
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
    let trianglesGood = vec![
        glam::uvec3(2,1,0), glam::uvec3(3,2,0),     //front side
        glam::uvec3(6,5,4), glam::uvec3(7,6,4),     //back side
        glam::uvec3(3,0,4), glam::uvec3(7,3,4),     //bottom side
        glam::uvec3(2,1,5), glam::uvec3(6,2,5),     //top side
        glam::uvec3(6,2,3), glam::uvec3(7,6,3),     //left side
        glam::uvec3(1,5,4), glam::uvec3(0,1,4),     //right side
        ];
    let verticesGood = vec![vertex0, vertex1, vertex2, vertex3, vertex4, vertex5, vertex6, vertex7];

    let mut mesh = Mesh::new();
    mesh.add_section_from_vertices(& trianglesGood, & verticesGood);

    let mut rot = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        clear_buffer(&mut buffer, 0);   //screen clear
        clear_buffer(&mut z_buffer, f32::INFINITY);

        let transform = Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, rot, 0.0, 0.0));

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
            
            

            raster_mesh(&mesh, &transform.local(), &camera.view(), &camera.projection(), Some(&texture), &mut buffer, &mut z_buffer, window_size);
            rot += 0.05;
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
