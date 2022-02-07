use glam::{Vec2, Vec3Swizzles};

//pub mod files. Important because this exposes these modules from other files to whoever uses lib.rs
pub mod geometry;
pub mod texture;
pub mod utils;
pub use {geometry::Vertex, geometry::Triangle, texture::Texture, utils::*};

#[cfg(test)] //unit tests in Rust
mod tests {
    use crate::geometry::Vertex;
    use crate::utils::*;

    #[test]
    fn lerping() {
        let v0 = Vertex { 
            position: glam::vec3(100.0, 100.0, 0.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v1 = Vertex {
            position: glam::vec3(100.0, 400.0, 0.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 1.0),
        };

        let interpolated = Lerp(v0, v1, 0.5);
        assert_eq!(interpolated.uv.y, 0.5);
    }
}

pub fn Raster_Triangle(tri: Triangle, buffer: &mut Vec<u32>, texture: &Texture, z_buffer: &mut Vec<f32>)
{
    for (i, pixel) in buffer.iter_mut().enumerate() 
    {
        let coords = index_to_coords(i, 500);
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
                //println!("{}", depth);

                let texCoords = bary.x * tri.vert0.uv + bary.y * tri.vert1.uv + bary.z * tri.vert2.uv;
                let color = texture.argb_at_uv(texCoords.x, texCoords.y);

                *pixel = color; //write to buffer
            }
        }
    }
}

pub fn Render_Depth(tri: Triangle, buffer: &mut Vec<u32>, z_buffer: &mut Vec<f32>)
{
    for (i, pixel) in buffer.iter_mut().enumerate() 
    {
        let coords = index_to_coords(i, 500);
        let coords = glam::vec2(coords.0 as f32, coords.1 as f32);

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
            let depth = bary.x * tri.vert0.position.z + bary.y * tri.vert1.position.z + bary.z * tri.vert2.position.z;
            if depth < z_buffer[i] {
                z_buffer[i] = depth;
                *pixel = (-depth * 255.0) as u32; //write to buffer

            }
        }
    }
}

pub fn Funky_Triangle(tri: Triangle, buffer: &mut Vec<u32>)
{
    let mut m0 : f32 = 0.0;
    let mut m1 : f32 = 0.0;
    let mut m2 : f32 = 0.0;

    for (i, pixel) in buffer.iter_mut().enumerate() 
    {
        let coords = index_to_coords(i, 500);
        let coords = glam::vec2(coords.0 as f32, coords.1 as f32);
        m0 = edge_function(tri.vert0.position.xy(), tri.vert1.position.xy(), coords);
        m1 = edge_function(tri.vert1.position.xy(), tri.vert2.position.xy(), coords);
        m2 = edge_function(tri.vert2.position.xy(), tri.vert0.position.xy(), coords);

        *pixel = to_argb8(
            255, 
            (m2*255.0) as u8, 
            (m0*255.0) as u8, 
            (m1*255.0) as u8,
        );
    }
}