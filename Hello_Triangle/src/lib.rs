use geometry::Mesh;
use glam::{Vec2, Vec3Swizzles, Mat4, Vec4};

//pub mod files. Important because this exposes these modules from other files to whoever uses lib.rs
pub mod geometry;
pub mod texture;
pub mod utils;
pub mod transform;
pub mod camera;
pub use {
    geometry::*, 
    texture::Texture, 
    transform::Transform, 
    camera::Camera,
    utils::*};

pub enum RenderType{
    Std,
    Depth,
    ClipDebug,
    Wireframe,
}

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

pub fn Raster_Triangle(tri: &Triangle, mvp: &Mat4, buffer: &mut Vec<u32>, texture: Option<&Texture>, z_buffer: &mut Vec<f32>, viewport_size: Vec2, rtype: &RenderType)
{
    //let mvp = *projection * *view * *model; //multiplied from right to left

    let clip0 = *mvp * Vec4::from((tri.vert0.position, 1.0));
    let clip1 = *mvp * Vec4::from((tri.vert1.position, 1.0));
    let clip2 = *mvp * Vec4::from((tri.vert2.position, 1.0));
    
    let rec0 = 1.0 / clip0.w;
    let rec1 = 1.0 / clip1.w;
    let rec2 = 1.0 / clip2.w;

    //Normalized Device Coordinates
    //perform perspective division to transform in ndc. xyz components of ndc are now between -1 and 1 (if within frustum)
    //normally the output of the vertex shader 
    let ndc0 = clip0 * rec0; //since clip.w is -1 to 1, map clip space coords to -1/1
    let ndc1 = clip1 * rec1;
    let ndc2 = clip2 * rec2;

    let v0 = tri.vert0 * rec0;
    let v1 = tri.vert1 * rec1;
    let v2 = tri.vert2 * rec2;

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

    if let Some(bb) = triangle_screen_bounding_box(&[sc0, sc1, sc2], viewport_size) {
        for y in (bb.top as usize)..=bb.bot as usize {
            for x in (bb.left as usize)..=bb.right as usize {
                let coords = glam::vec2(x as f32, y as f32) + 0.5;
                let pixel_id = coords_to_index(x, y, viewport_size.x as usize);
                let area = edge_function(sc0, sc1, sc2);

                if let Some(bary) = Barycentric_Coordinates(coords,sc0,sc1,sc2, area) {
                    let correction = bary.x * rec0 + bary.y * rec1 + bary.z * rec2;
                    let correction = 1.0 / correction;
                    let depth = bary.x * ndc0.z + bary.y * ndc1.z + bary.z * ndc2.z;
                    if depth < z_buffer[pixel_id] {
                        z_buffer[pixel_id] = depth;


                        let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                        let color = color * correction;
                        let mut color = to_argb8(
                            255, 
                            (color.x * 255.0) as u8,
                            (color.y * 255.0) as u8,
                            (color.z * 255.0) as u8,
                        );

                        if let Some(tex) = texture {
                            let texCoords = bary.x * v0.uv + bary.y * v1.uv + bary.z * v2.uv;
                            let texCoords = texCoords * correction;
                            color = tex.argb_at_uv(texCoords.x, texCoords.y);
                        }
                        
                        if let RenderType::Depth = rtype {
                            color = to_argb8(
                                255,
                                (depth * 255.0) as u8,
                                (depth * 255.0) as u8,
                                (depth * 255.0) as u8,
                            );
                        }

                        buffer[pixel_id] = color; //write to buffer
                    }

                }   
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

pub fn raster_mesh(
    mesh: &Mesh,
    mvp: &Mat4,
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
    render_type: &RenderType,
) {
    for tri in mesh.triangles() {
        let vertices = mesh.get_vertices_from_triangle(*tri);

        let tempTri = &Triangle {    //what the fuck?
            vert0: *vertices[0],
            vert1: *vertices[1],
            vert2: *vertices[2],
        };

        Raster_Triangle(tempTri, mvp, buffer, texture, z_buffer, viewport_size, render_type)
    }
}

pub fn triangle_screen_bounding_box(
    tri: &[Vec2; 3], //not triangle struct because this should be used with screen coordinates
    viewport_size: Vec2,
) -> Option<BoundingBox2D> {
    let bb = get_triangle_bounding_box_2d(tri);

    //just AABB
    if bb.left >= viewport_size.x || bb.right < 0.0 || bb.bot >= viewport_size.y || bb.top < 0.0 {
        None
    } else {
        let left = bb.left.max(0.0);
        let right = bb.right.min(viewport_size.x - 1.0);
        let top = bb.top.min(viewport_size.y - 1.0);
        let bot = bb.bot.max(0.0);

        Some(BoundingBox2D {
            left,
            right,
            top,
            bot,
        })
    }
}