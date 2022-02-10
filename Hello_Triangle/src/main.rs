extern crate minifb;
use std::path::Path;
use std::thread;
use std::time::Instant;
use std::vec;

use glam::{f64, Vec3Swizzles};
use glam::{Vec2, Vec3, Vec3A, Vec4};
use minifb::{Key, Window, WindowOptions};
use Hello_Triangle::geometry::Mesh;

use Hello_Triangle::*;

const WIDTH: usize = 480;
const HEIGHT: usize = 480;

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

    let tile_size_xy = 32;

    if WIDTH % tile_size_xy != 0 || HEIGHT % tile_size_xy != 0 {
        let err_info =
            "Tile size is incompatible with set screen size. Needs to be wholly divisable";
        //error!("Error: {}", err_info);
        eprintln!("Error: {}", err_info);
    }

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    //thread builder, could build worker threads that rasterize specific cell
    //can set stack size of thread through builder, so maybe max mem per pixel * 32^2 for a tile
    //for 480x480 res you need 15 tiles

    //spawn threads
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread number from spawned threads {}", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number of threads from main thread {}", i);
        thread::sleep(std::time::Duration::from_millis(1));
    }
    handle.join().unwrap();

    let mut delta_time = 0.0;
    let mut buffer: Vec<u32> = vec![to_argb8(255, 0, 0, 0); WIDTH * HEIGHT];
    let mut z_buffer = vec![f32::INFINITY; WIDTH * HEIGHT];

    let mut frame_times: Vec<f32> = vec![0.0; 60];

    let mut rendering_type = RenderType::Std;

    let window_size = glam::vec2(WIDTH as f32, HEIGHT as f32);

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;

    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 8.0)),
        frustum_near: 0.1,
        frustum_far: 100.0,
        ..Default::default()
    };

    //maybe multithread model loading, could be fun
    let texture = Texture::Load(Path::new("Assets/Helmet/Default_albedo.jpg"));
    let mesh = load_gltf(Path::new("Assets/Helmet/DamagedHelmet.gltf"));

    let count = 0;
    //let mut tiles: Vec<Tile> = vec![Tile::new(); buffer.iter().len() / 32];

    //construct tiles
    // for i in buffer.iter() {
    //     let coords = index_to_coords(count, WIDTH);
    //     if coords.0 % tile_size_xy == 0 && coords.1 % tile_size_xy == 0 {
    //         //tiles.append(Tile {});
    //         //spawn new thread for tile... how to manage?
    //         let handle = thread::spawn(|| {});
    //     }
    // }

    let mut rot = 0.0;

    let mut cam_rot = 0.0;

    let time_tracker = Instant::now();
    let mut last: u128 = 0;
    let mut now: u128 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        //start counting deltatime
        now = time_tracker.elapsed().as_millis();
        delta_time = (now - last) as f32 * 0.001;
        last = now;

        if (now * 60) % 1 == 0 {
            println!("Framerate: {:?} p/sec", (delta_time * 60.0));
        }

        clear_buffer(&mut buffer, 0); //screen clear
        clear_buffer(&mut z_buffer, f32::INFINITY);

        camera.transform = Transform::from_translation_rotation(
            camera.transform.translation,
            glam::Quat::from_euler(glam::EulerRot::XYZ, 0.0, cam_rot, 0.0),
        );

        if window.is_key_down(Key::Space) {
            rendering_type = RenderType::Depth;
        } else {
            rendering_type = RenderType::Std;
        }

        camera_input(&window, &mut camera, &delta_time);

        if window.is_key_down(Key::A) {
            cam_rot += 0.5 * delta_time;
        }
        if window.is_key_down(Key::D) {
            cam_rot -= 0.5 * delta_time;
        }

        let parent_local = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            rot * 0.8,
            rot * 0.5,
            rot,
        ))
        .local();
        let view = camera.view();
        let proj = camera.projection();

        //should prolly foreach mesh this
        raster_mesh(
            &mesh,
            &parent_local,
            &(proj * view * parent_local),
            Some(&texture),
            &mut buffer,
            &mut z_buffer,
            window_size,
            &rendering_type,
        );

        rot += 0.6 * delta_time;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        count + 1;
    }
}

// pub fn frame_start(start_time: u128, end_time: u128, frame_times: Vec<f32>) {
//     //find frame length (delta between start and end)
//     //frame_times.append((start_time - end_time))

//     //add to a queue? clear queue every second?

//     //before queue clear print amount of frames
// }

pub fn camera_input(window: &Window, camera: &mut Camera, delta_time: &f32) {
    let mut translate_axis = glam::vec3(0.0, 0.0, 0.0);
    let mut rotate_euler = glam::vec3(0.0, 0.0, 0.0);

    if (window.is_key_down(Key::Up)) {
        translate_axis.z += 3.0 * *delta_time;
    }
    if (window.is_key_down(Key::Down)) {
        translate_axis.z -= 3.0 * *delta_time;
        //pos.z += 3.0 * delta_time;
    }
    if (window.is_key_down(Key::Left)) {
        translate_axis.x -= 3.0 * *delta_time;
        //pos.x -= 3.0 * delta_time;
    }
    if (window.is_key_down(Key::Right)) {
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

    camera.transform.translation += (camera.transform.right() * camera.speed * translate_axis.x
        + camera.transform.forward() * camera.speed * translate_axis.z
        + camera.transform.up() * camera.speed * translate_axis.y)
        * *delta_time;

    //camera.transform.rotation += glam::Quat(glam::EulerRot::XYZ, rotate_euler.x, rotate_euler.y, rotate_euler.z);
}
