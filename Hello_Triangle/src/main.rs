extern crate minifb;
use std::os::windows;
use std::time::Instant;
use std::vec;
use std::path::Path;


use Hello_Triangle::geometry::Mesh;
use glam::{Vec3Swizzles, f64};
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

    let mut delta_time = 0.0;
    let mut buffer: Vec<u32> = vec![to_argb8(255, 0, 0, 0); WIDTH * HEIGHT];
    let mut z_buffer = vec![f32::INFINITY; WIDTH * HEIGHT];
    
    let mut rendering_type = RenderType::Std;

    let window_size = glam::vec2(WIDTH as f32, HEIGHT as f32);

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    
    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 8.0)),
        frustum_far: 100.0,
        ..Default::default()
    };

    let texture = Texture::Load(Path::new("D:/BUAS/MC/Rust/RasterRusterMC/Hello_Triangle/Assets/bojan.jpg"));

    let vertex0 = Vertex{
        position: glam::vec4(-1.0, -1.0, 1.0, 1.0),
        normal: glam::vec3(0.0, 0.0, 0.0),
        color: glam::vec3(1.0, 0.0, 0.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let vertex1 = Vertex{
        position: glam::vec4(-1.0, 1.0, 1.0, 1.0),
        normal: glam::vec3(0.0, 0.0, 0.0),
        color: glam::vec3(0.0, 1.0, 0.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let vertex2 = Vertex {
        position: glam::vec4(1.0, 1.0, 1.0, 1.0),
        normal: glam::vec3(0.0, 0.0, 0.0),
        color: glam::vec3(0.0, 0.0, 1.0),
        uv: glam::vec2(1.0, 0.0),
    };
    let vertex3 = Vertex {
        position: glam::vec4(1.0, -1.0, 1.0, 1.0),
        normal: glam::vec3(0.0, 0.0, 0.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
    };

    //let mut triangles: [Triangle; 2] = [Triangle::default(); 2];

    //println!("interpolated verted: {:?}", Lerp(triangles[0].vert0, Lerp(triangles[0].vert1, 0.5)); //explodes
    let trianglesGood = vec![glam::uvec3(2,1,0), glam::uvec3(3,2,0)];
    let verticesGood = vec![vertex0, vertex1, vertex2, vertex3];

    let mut mesh = Mesh::new();
    mesh.add_section_from_vertices(& trianglesGood, & verticesGood);

    let mut rot = 0.0;

    let transform0 = Transform::IDENTITY;
    let transform1 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ, 
        -std::f32::consts::PI, 
        0.0, 
        0.0,
    ));
    let transform2 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ, 
        std::f32::consts::FRAC_PI_2, 
        0.0, 
        0.0,
    ));
    let transform3 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ, 
        -std::f32::consts::FRAC_PI_2, 
        0.0, 
        0.0,
    ));
    let transform4 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ, 
        0.0, 
        -std::f32::consts::FRAC_PI_2, 
        0.0,
    ));
    let transform5 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ, 
        0.0, 
        std::f32::consts::FRAC_PI_2, 
        0.0,
    ));

    //let mut pos = glam::Vec3::new(0.0, 0.0, 8.0);
    let mut cam_rot = 0.0;

    let mut standard_rendering = true;

    let time_tracker = Instant::now();
    let mut last : u128 = 0;
    let mut now: u128 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        //start counting deltatime
        now = time_tracker.elapsed().as_millis();
        //println!("this frame {:?}, last frame {:?}", now, last);
        delta_time = (now - last) as f32 * 0.001;
        last = now;
        
        //println!("delta time {:?}", delta_time);

        //now = currTotal - last;

        //delta_time = (now - last) * 1000 / now;
        
        //now += time_tracker.elapsed().as_millis();
        //let total_elapsed = now.elapsed().as_millis();
        //last = now - time_tracker.elapsed().as_millis();



        //println!("{:?}", elapsed.as_millis());
        clear_buffer(&mut buffer, 0);   //screen clear
        clear_buffer(&mut z_buffer, f32::INFINITY);
        
        //camera.transform = Transform::from_translation(pos);
        camera.transform = Transform::from_translation_rotation(camera.transform.translation, glam::Quat::from_euler(glam::EulerRot::XYZ, 0.0, cam_rot, 0.0));
        //let mut pos = glam::Vec3(0.0, 0.0, 0.0);
        //println!("{:?}", &pos);
        //println!("{:?}", &camera.transform.forward());

        if(window.is_key_down(Key::Space)){
            standard_rendering = false;
            rendering_type = RenderType::Depth;
        }else {
            standard_rendering = true;
            rendering_type = RenderType::Std;
        }

        camera_input(&window, &mut camera, &delta_time);

        if(window.is_key_down(Key::A)){
            cam_rot += 0.5 * delta_time;
        }
        if(window.is_key_down(Key::D)){
            cam_rot -= 0.5 * delta_time;
        }


        //pos *= camera.transform.local();
        //let cam_trans_mat = 
        let parent_local = Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, rot * 0.8, rot * 0.5, rot)).local();
        let view = camera.view();
        let proj = camera.projection();
        
        raster_mesh(&mesh, &parent_local, &(proj * view * parent_local * transform0.local()),Some(&texture), &mut buffer, &mut z_buffer, window_size, &rendering_type);
        raster_mesh(&mesh, &parent_local, &(proj * view * parent_local * transform1.local()), Some(&texture), &mut buffer, &mut z_buffer, window_size, &rendering_type);
        raster_mesh(&mesh, &parent_local, &(proj * view * parent_local * transform2.local()), Some(&texture), &mut buffer, &mut z_buffer, window_size, &rendering_type);
        raster_mesh(&mesh, &parent_local, &(proj * view * parent_local * transform3.local()), Some(&texture), &mut buffer, &mut z_buffer, window_size, &rendering_type);
        raster_mesh(&mesh, &parent_local, &(proj * view * parent_local * transform4.local()), Some(&texture), &mut buffer, &mut z_buffer, window_size, &rendering_type);
        raster_mesh(&mesh, &parent_local, &(proj * view * parent_local * transform5.local()), Some(&texture), &mut buffer, &mut z_buffer, window_size, &rendering_type);
        rot += 0.6 * delta_time;

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

pub fn camera_input(window: &Window, camera: &mut Camera, delta_time: &f32) {
    let mut translate_axis = glam::vec3(0.0, 0.0, 0.0);
    let mut rotate_euler = glam::vec3(0.0, 0.0, 0.0);

    if(window.is_key_down(Key::Up)){
        translate_axis.z += 3.0 * *delta_time;
    }
    if(window.is_key_down(Key::Down)){
        translate_axis.z -= 3.0 * *delta_time;
        //pos.z += 3.0 * delta_time;
    }
    if(window.is_key_down(Key::Left)){
        translate_axis.x -= 3.0 * *delta_time;
        //pos.x -= 3.0 * delta_time;
    }
    if(window.is_key_down(Key::Right)){
        translate_axis.x += 3.0 * *delta_time;
        //pos.x += 3.0 * delta_time;
    }
    //if(window.is_key_down(Key::A)){
    //    //cam_rot += 0.5 * delta_time;
    //    rotate_euler += 0.5 * *delta_time;
    //}
    //if(window.is_key_down(Key::D)){
    //    //cam_rot -= 0.5 * delta_time;
    //    rotate_euler -= 0.5 * *delta_time;
    //}

    camera.transform.translation += 
        (camera.transform.right() * camera.speed * translate_axis.x + 
        camera.transform.forward() * camera.speed * translate_axis.z +
        camera.transform.up() * camera.speed * translate_axis.y) * *delta_time;

    //camera.transform.rotation += glam::Quat(glam::EulerRot::XYZ, rotate_euler.x, rotate_euler.y, rotate_euler.z);
}
