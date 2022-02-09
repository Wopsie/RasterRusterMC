use glam::{Vec2, Vec3Swizzles, Mat4, Vec4, Vec4Swizzles, Vec3};
use std::path::Path;
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

pub enum ClipResult {
    None,
    One(Triangle),
    Two((Triangle, Triangle)),
}

#[cfg(test)] //unit tests in Rust
mod tests {
    use crate::geometry::Vertex;
    use crate::transform::{Transform, TransformInitialParams};
    use crate::utils::*;

    #[test]
    fn lerping() {
        let v0 = Vertex { 
            position: glam::vec4(100.0, 100.0, 0.0, 1.0),
            normal: glam::vec3(0.0, 0.0, 1.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };
             let v1 = Vertex {
            position: glam::vec4(100.0, 400.0, 0.0, 1.0),
            normal: glam::vec3(0.0, 0.0, 1.0),
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

pub fn Raster_Clipped_Triangle(
    tri: &Triangle, 
    buffer: &mut Vec<u32>, 
    texture: Option<&Texture>, 
    z_buffer: &mut Vec<f32>, 
    viewport_size: Vec2, 
    rtype: &RenderType)
{
    let rec0 = 1.0 / tri.vert0.position.w;
    let rec1 = 1.0 / tri.vert1.position.w;
    let rec2 = 1.0 / tri.vert2.position.w;

    //Normalized Device Coordinates
    //perform perspective division to transform in ndc. xyz components of ndc are now between -1 and 1 (if within frustum)
    //normally the output of the vertex shader 
    let ndc0 = tri.vert0.position * rec0; //since clip.w is -1 to 1, map clip space coords to -1/1
    let ndc1 = tri.vert1.position * rec1;
    let ndc2 = tri.vert2.position * rec2;

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

                        let normal = bary.x * v0.normal + bary.y * v1.normal + bary.z * v2.normal;
                        let normal = normal * correction;

                        let n_dot_l = normal.dot(Vec3::ONE.normalize());    //normalize vertex normals

                        let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                        let mut color = color * correction;

                        
                        if let Some(tex) = texture {
                            let texCoords = bary.x * v0.uv + bary.y * v1.uv + bary.z * v2.uv;
                            let texCoords = texCoords * correction;
                            color = tex.argb_at_uvf(texCoords.x, texCoords.y).yzw();
                        }
                        
                        let ambient = glam::vec3(0.2, 0.2, 0.2);
                        color = color * n_dot_l + ambient;
                        let mut out_color = to_argb8(
                            255, 
                            (color.x * 255.0) as u8,
                            (color.y * 255.0) as u8,
                            (color.z * 255.0) as u8,
                        );

                        if let RenderType::Depth = rtype {
                            out_color = to_argb8(
                                255,
                                (depth * 255.0) as u8,
                                (depth * 255.0) as u8,
                                (depth * 255.0) as u8,
                            );
                        }

                        buffer[pixel_id] = out_color; //write to buffer
                    }

                }   
            }
        }
    }
}

pub fn Raster_Triangle(
    tri: &Triangle,
    model_mat: &Mat4,
    mvp: &Mat4, 
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
    rtype: &RenderType,
){
    let cof_mat = cofactor(model_mat);

    let mut clip_tri = tri.transform(mvp);
    clip_tri.vert0.normal = (cof_mat * tri.vert0.normal.extend(0.0)).xyz();
    clip_tri.vert1.normal = (cof_mat * tri.vert1.normal.extend(0.0)).xyz();
    clip_tri.vert2.normal = (cof_mat * tri.vert2.normal.extend(0.0)).xyz();

    match clip_cull_triangle(&clip_tri) {
        ClipResult::None => {} //lookup lambda in rust
        ClipResult::One(ctri) => {
            Raster_Clipped_Triangle(&ctri, buffer, texture, z_buffer, viewport_size, rtype);   
        }
        ClipResult::Two(ctri) => {
            Raster_Clipped_Triangle(&ctri.0, buffer, texture, z_buffer, viewport_size, rtype);
            Raster_Clipped_Triangle(&ctri.1, buffer, texture, z_buffer, viewport_size, rtype);
        }
    }
}

//View Frustum Culling
pub fn cull_triangle_view_frustum(tri: &Triangle) -> bool {
    // cull tests against the 6 planes
    if tri.vert0.position.x > tri.vert0.position.w
        && tri.vert1.position.x > tri.vert1.position.w
        && tri.vert2.position.x > tri.vert2.position.w
    {
        return true;
    }
    if tri.vert0.position.x < -tri.vert0.position.w
        && tri.vert1.position.x < -tri.vert1.position.w
        && tri.vert2.position.x < -tri.vert2.position.w
    {
        return true;
    }
    if tri.vert0.position.y > tri.vert0.position.w
        && tri.vert1.position.y > tri.vert1.position.w
        && tri.vert2.position.y > tri.vert2.position.w
    {
        return true;
    }
    if tri.vert0.position.y < -tri.vert0.position.w
        && tri.vert1.position.y < -tri.vert1.position.w
        && tri.vert2.position.y < -tri.vert2.position.w
    {
        return true;
    }
    if tri.vert0.position.z > tri.vert0.position.w
        && tri.vert1.position.z > tri.vert1.position.w
        && tri.vert2.position.z > tri.vert2.position.w
    {
        return true;
    }
    if tri.vert0.position.z < 0.0 && tri.vert1.position.z < 0.0 && tri.vert2.position.z < 0.0
    {
        return true;
    }

    false
}

pub fn cull_triangle_backface(tri: &Triangle) -> bool {
    let normal = tri.vert1.position.xyz() - tri.vert0.position.xyz().cross(tri.vert2.position.xyz() - tri.vert0.position.xyz());
    let view_dir = -Vec3::Z;

    normal.dot(view_dir) >= 0.0
}

fn clip_triangle_two(tri: &Triangle) -> (Triangle, Triangle) {
    let alpha_a = (-tri.vert0.position.z) / (tri.vert1.position.z - tri.vert0.position.z);
    let alpha_b = (-tri.vert0.position.z) / (tri.vert2.position.z - tri.vert0.position.z);

    //interpolate vertices 
    let mut v0a = Lerp(tri.vert0, tri.vert1, alpha_a);
    let mut v0b = Lerp(tri.vert0, tri.vert2, alpha_b);

    let green = Vec3::new(0.0, 1.0, 0.0);
    let blue = Vec3::new(0.0, 0.0, 1.0);
    
    let mut result_a = *tri;
    let mut result_b = *tri;

    result_a.vert0 = v0a;

    result_b.vert0 = v0a;
    result_b.vert1 = v0b;

    result_a.vert0.color = green;
    result_a.vert1.color = green;
    result_a.vert2.color = green;
    result_b.vert0.color = blue;
    result_b.vert1.color = blue;
    result_b.vert2.color = blue;

    (result_a, result_b)    //not sure what type this actually is. Vector? Array? Tuple?
}

fn clip_triangle_one(tri: &Triangle) -> Triangle {
    let alpha_a = (-tri.vert0.position.z) / (tri.vert2.position.z - tri.vert0.position.z);
    let alpha_b = (-tri.vert1.position.z) / (tri.vert2.position.z - tri.vert1.position.z);

    //interpolate vertices 
    let mut v0 = Lerp(tri.vert0, tri.vert2, alpha_a);
    let mut v1 = Lerp(tri.vert1, tri.vert2, alpha_b);
    let mut v2 = tri.vert2;

    let red = Vec3::new(1.0, 0.0, 0.0);

    v0.color = red;
    v1.color = red;
    v2.color = red;

    //if Triangle.vert0.. was called v0.. you would not need to explicitly assign v0.. to vert0.. 
    Triangle {vert0 : v0, vert1 : v1, vert2 : v2}
}

pub fn clip_cull_triangle(tri: &Triangle) -> ClipResult {
    if cull_triangle_backface(tri) {
        //triangle gets culled
        return ClipResult::None;    //why does this have to be an explicit return statement?
    }
    if cull_triangle_view_frustum(tri) {
        ClipResult::None
    } else {
        if tri.vert0.position.z < 0.0 {

            if tri.vert1.position.z < 0.0 {
                ClipResult::One(clip_triangle_one(tri))
            } else if tri.vert2.position.z < 0.0 {
                ClipResult::One(clip_triangle_one(&tri.reorder(VerticesOrder::ACB)))
            } else {
                ClipResult::Two(clip_triangle_two(&tri.reorder(VerticesOrder::ACB)))
            }

        } else if tri.vert1.position.z < 0.0 {
            
            if tri.vert2.position.z < 0.0 {
                ClipResult::One(clip_triangle_one(&tri.reorder(VerticesOrder::BCA)))
            } else {
                ClipResult::Two(clip_triangle_two(&tri.reorder(VerticesOrder::BAC)))
            }

        } else if tri.vert2.position.z < 0.0 {
            ClipResult::Two(clip_triangle_two(&tri.reorder(VerticesOrder::CBA)))
        } else {
            ClipResult::One(*tri)
        }
    }
}

pub fn raster_mesh(
    mesh: &Mesh,
    loc_mat: &Mat4,
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

        Raster_Triangle(
            tempTri, 
            loc_mat,
            mvp, 
            texture, 
            buffer, 
            z_buffer, 
            viewport_size, 
            render_type
        );
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

pub fn load_gltf(path: &Path) -> Mesh {
    let (document, buffers, _images) = gltf::import(path).unwrap();

    for scene in document.scenes() {
        for node in scene.nodes() {
            println!(
                "Node #{} has {} children, camera: {:?}, mesh: {:?}, transform: {:?}",
                node.index(),
                node.children().count(),
                node.camera(),
                node.mesh().is_some(),
                node.transform(),
            );
            println!(
                "Node #{} has transform: trans {:?}, rot {:?}, scale {:?},",
                node.index(),
                node.transform().decomposed().0,
                node.transform().decomposed().1,
                node.transform().decomposed().2,
            );
            if let Some(mesh) = node.mesh() {
                return Mesh::load_from_gltf(&mesh, &buffers); //why does this need an explicit return statement?
            }
        }
    }

    Mesh::new()
}