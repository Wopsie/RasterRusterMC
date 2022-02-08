//use std::process::Output;

use std::process::Output;

use glam::{Vec2, Vec3};
//pub mod geometry;
//pub use geometry::Point;
//
use crate::{geometry::Point};

pub fn Barycentric_Coordinates(
    point: Vec2,
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
    area: f32,
) -> Option<Vec3> {
    let m0 = edge_function(point, v1, v2);
    let m1 = edge_function(point, v2, v0);
    let m2 = edge_function(point, v0, v1);

    let a = 1.0 / area;
    if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
        Some(glam::vec3(m0 * a, m1 * a, m2 * a))
    } else {
        None
    }
}

pub fn Lerp<T>(start: T, end: T, alpha: f32) -> T 
where //wtf?
    T: std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    start + (end - start) * alpha
}

//actually just cross product? or determinant?
pub fn edge_function(v0: Vec2, v1: Vec2, p: Vec2) -> f32 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub fn index_to_coords(p: usize, width: usize) -> (usize, usize) {
    (p % width, p / width)
}

pub fn coords_to_index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

//convert colors to u32
pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32; //a
    argb = (argb << 8) + r as u32; //r
    argb = (argb << 8) + g as u32; //g
    argb = (argb << 8) + b as u32; //b
    argb //returns argb. Syntax funkyness
         //you can do "shadowing" on the data(?)
         //by default statics cannot be mutable
         //same for constants
         //need to use "unsafe" for that
}

pub fn clear_buffer<T>(buffer: &mut Vec<T>, value: T)
where
    T: Copy,
{
    buffer.iter_mut().map(|x| *x = value).count();
}

pub fn map_to_range<T>(v: T, a1: T, a2: T, b1: T, b2: T) -> T
where //"where" is used to require generic type "T" to have implemented various operators and rust Traits
    T: std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{    
    b1 + (v - a1) * (b2 - b1) / (a2 - a1)
}

pub fn bresenham_function(vert0: Vec2, vert1: Vec2) -> Vec<Point>
//pub fn bresenham_function(vert0: Vec2, vert1: Vec2, pixel: Vec2) -> f32
{
    let mut coords: Vec<Point> = vec![];

    //let i0 = coords_to_index(vert0.x as usize, vert0.y as usize, HEIGHT);
    //let i1 = coords_to_index(vert1.x as usize, vert1.y as usize, HEIGHT);

    //let v0ScreenCoords = index_to_coords(i0, HEIGHT);
    //let v1ScreenCoords = index_to_coords(i1, HEIGHT);
    //Maybe print this to see if the coords are actually different

    //these should be in screen coordinates. i32s
    let deltax: i32 = i32::abs(vert1.x as i32 - vert0.x as i32);
    let deltay: i32 = i32::abs(vert1.y as i32 - vert0.y as i32);

    let stepX: i32 = if vert0.x < vert1.x {1}else{-1};
    let stepY: i32 = if vert0.y < vert1.y {1}else{-1};

    let mut currX: i32 = vert0.x as i32;
    let mut currY: i32 = vert0.y as i32;

    let mut error = if deltax > deltay {deltax} else {-deltay} /2;

    loop {
        coords.push(Point{x: currX, y: currY});

        if currX == vert1.x as i32 && currY == vert1.y as i32
        {
            break;
        }

        let error2: i32 = error;
        if error2 > -deltax
        {
            error -= deltay;
            currX += stepX;
        }

        if error2 < deltay
        {
            error += deltay;
            currY += stepY;
        }
    }

    coords
}
