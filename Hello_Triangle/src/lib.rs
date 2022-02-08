use geometry::Mesh;
use glam::{Vec2, Vec3Swizzles, Mat4, Vec4};

//pub mod files. Important because this exposes these modules from other files to whoever uses lib.rs
pub mod geometry;
pub mod texture;
pub mod utils;
pub mod transform;
pub mod camera;
pub use {
    geometry::Vertex, 
    geometry::Triangle, 
    texture::Texture, 
    transform::Transform, 
    camera::Camera,
    utils::*};

#[cfg(test)] //unit tests in Rust
mod tests {
    use crate::geometry::Vertex;
    use crate::transform::{Transform, TransformInitialParams};
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
    
    #[test]
    fn transform_init() {
        let translation = glam::vec3(1.2, 199.0, 9.0);
        let rotation = glam::Quat::from_rotation_z(std::f32::consts::PI / 2.0);
        let transform = Transform::from(TransformInitialParams::TranslationRotation( 
            translation,
            rotation,
        ));
             assert_eq!(transform.translation.x, translation.x);
    }
}

pub fn Raster_Triangle(tri: Triangle, model: &Mat4, view: &Mat4, projection: &Mat4, buffer: &mut Vec<u32>, texture: Option<&Texture>, z_buffer: &mut Vec<f32>, viewport_size: Vec2)
{
    let mvp = *projection * *view * *model; //multiplied from right to left

    let clip0 = mvp * Vec4::from((tri.vert0.position, 1.0));
    let clip1 = mvp * Vec4::from((tri.vert1.position, 1.0));
    let clip2 = mvp * Vec4::from((tri.vert2.position, 1.0));
    
    //Normalized Device Coordinates
    //perform perspective division to transform in ndc. xyz components of ndc are now between -1 and 1 (if within frustum)
    //normally the output of the vertex shader 
    let ndc0 = clip0 / clip0.w; //since clip.w is -1 to 1, map clip space coords to -1/1
    let ndc1 = clip1 / clip1.w;
    let ndc2 = clip2 / clip2.w;

    //remap NDC (-1/1) xy axes to viewport size (width/height)
    let sc0 = glam::vec2 (
        map_to_range(ndc0.x, -1.0, 1.0, 0.0, viewport_size.x),
        map_to_range(-ndc0.y, -1.0, 1.0, 0.0, viewport_size.y),
    );

    let sc1 = glam::vec2 (
        map_to_range(ndc1.x, -1.0, 1.0, 0.0, viewport_size.x),
        map_to_range(-ndc1.y, -1.0, 1.0, 0.0, viewport_size.y),
    );

    let sc2 = glam::vec2(
        map_to_range(ndc2.x, -1.0, 1.0, 0.0, viewport_size.x),
        map_to_range(-ndc2.y, -1.0, 1.0, 0.0, viewport_size.y),
    );

    for (i, pixel) in buffer.iter_mut().enumerate() 
    {
        let coords = index_to_coords(i, viewport_size.y as usize);
        //shadowing a variable
        let coords = glam::vec2(coords.0 as f32, coords.1 as f32) + 0.5;
        let area = edge_function(sc0, sc1, sc2);

        if let Some(bary) = Barycentric_Coordinates(coords,sc0,sc1,sc2, area) {
            // bary var presumably contains barycentric coordinates of the given coords on the given triangle
            let depth = bary.x * tri.vert0.position.z + bary.y * tri.vert1.position.z + bary.z * tri.vert2.position.z;
            if depth < z_buffer[i] {
                z_buffer[i] = depth;
                
                let color = bary.x * tri.vert0.color + bary.y * tri.vert1.color + bary.z * tri.vert2.color;

                let mut color = to_argb8(
                    255, 
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8,
                );

                if let Some(tex) = texture {
                    let texCoords = bary.x * tri.vert0.uv + bary.y * tri.vert1.uv + bary.z * tri.vert2.uv;
                    color = tex.argb_at_uv(texCoords.x, texCoords.y);
                }

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

pub fn raster_mesh(
    mesh: &Mesh,
    model_mat: &Mat4,
    view_mat: &Mat4,
    projection_mat: &Mat4,
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
) {
    for tri in mesh.triangles() {
        let vertices = mesh.get_vertices_from_triangle(*tri);

        let tempTri = Triangle {    //what the fuck?
            vert0: *vertices[0],
            vert1: *vertices[1],
            vert2: *vertices[2],
        };

        Raster_Triangle(tempTri, model_mat, view_mat, projection_mat, buffer, texture, z_buffer, viewport_size)
    }
}