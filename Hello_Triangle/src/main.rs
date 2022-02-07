extern crate minifb;
use core::prelude::v1;

use glam::Vec3Swizzles;
use glam::{Vec2, Vec3, Vec3A, Vec4};
use minifb::{Key, Window, WindowOptions};

pub mod utils;
pub use utils::*;
pub mod geometry;
pub use geometry::Vertex;
pub use geometry::Triangle;
pub use geometry::Point;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

// pub fn Raster_Triangle(tri: Triangle, buffer: &mut Vec<u32>)
// {
//     for (i, pixel) in buffer.iter_mut().enumerate() 
//     {
//         let coords = index_to_coords(i, HEIGHT);
//         //shadowing a variable
//         let coords = glam::vec2(coords.0 as f32, coords.1 as f32);
//         let area = edge_function(
//             tri.vert0.position.xy(), 
//             tri.vert1.position.xy(), 
//             tri.vert2.position.xy(),
//         );

//         //if let Some(bary) = Barycentric_Coordinates()
//     }
// }

fn main() {
    let mut buffer: Vec<u32> = vec![to_argb8(255, 255, 0, 0); WIDTH * HEIGHT];

    let mut redTriangle: bool = true;
    let mut wireFrameRend: bool = false;

    let mut window = Window::new(
        "Test - Do not press 'W'",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });


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

    //let mut triangles = vec![0; 2];
    //triangles.push(triangle1);

    let mut triangles: [Triangle; 1] = [triangle1; 1];

    triangles[0] = Triangle {
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

        let mut points: Vec<Point> = Vec::new();


        if wireFrameRend
        {
            //     // do bresenham here
            for i in triangles.iter() {
                points.append(&mut bresenham_function(i.vert0.position.xy(), i.vert1.position.xy()));
                points.append(&mut bresenham_function(i.vert1.position.xy(), i.vert2.position.xy()));
                points.append(&mut bresenham_function(i.vert2.position.xy(), i.vert0.position.xy()));

            }

            //     pass mutable reference to bresenham function? WIP
        } //else do buffer loop

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
                m0 = edge_function(triangles[0].vert0.position.xy(), triangles[0].vert1.position.xy(), coords);
                m1 = edge_function(triangles[0].vert1.position.xy(), triangles[0].vert2.position.xy(), coords);
                m2 = edge_function(triangles[0].vert2.position.xy(), triangles[0].vert0.position.xy(), coords);
            } else {
                //do bresenham here

                //m0 = bresenham_function(triangles[0].vert0.position.xy(), triangles[0].vert1.position.xy(), coords);
                //m1 = bresenham_function(triangles[0].vert1.position.xy(), triangles[0].vert2.position.xy(), coords);
                //m2 = bresenham_function(triangles[0].vert2.position.xy(), triangles[0].vert0.position.xy(), coords);
                //become the lines, not the edge, does not get filled

                //test against contents of points vector
                
                //this is crashes
                for point in points.iter() {
                    let ip = coords_to_index(point.x as usize, point.y as usize, HEIGHT);
                    
                    let is_point_in_line = points.iter().any(| point| ip == count);

                    if is_point_in_line{
                        m0 = 1.0;
                        m1 = 1.0;
                        m2 = 1.0;
                    }else {
                        m0 = 0.0;
                        m1 = 0.0;
                        m2 = 0.0;
                    }
                }
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
