use glam::{Mat4, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

pub struct BoundingBox2D {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bot: f32,
}

pub fn get_triangle_bounding_box_2d(tri: &[Vec2; 3]) -> BoundingBox2D {
    let left = tri[0].x.min(tri[1].x).min(tri[2].x);
    let right = tri[0].x.max(tri[1].x).max(tri[2].x);
    let top = tri[0].y.min(tri[1].y).min(tri[2].y);
    let bot = tri[0].y.max(tri[1].y).max(tri[2].y);

    BoundingBox2D {
        left,
        right,
        top,
        bot,
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

//data struct
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vec4,
    pub normal: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}

//implementation, bind functions to Vertex data struct
impl Vertex {
    //return self, like a constructor kindof
    pub fn Construct(position: Vec4, normal: Vec3, color: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            color,
            uv,
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Vec4::new(0.0, 0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            color: Vec3::new(0.0, 0.0, 0.0),
            uv: Vec2::new(1.0, 1.0),
        }
    }
}

//implements Add trait for vertex. Implementation is custom, but Add trait is generic. Traits are used to identify if generic objects implement certain functionality.
impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let position = self.position + rhs.position;
        let normal = self.normal + rhs.normal;
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self::Construct(position, normal, color, uv)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let position = self.position - rhs.position;
        let normal = self.normal - rhs.normal;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self::Construct(position, normal, color, uv)
    }
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.position * rhs;
        let normal = self.normal * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self::Construct(position, normal, color, uv)
    }
}

impl MulAssign<f32> for Vertex {
    fn mul_assign(&mut self, rhs: f32) {
        self.position *= rhs;
        self.normal *= rhs;
        self.color *= rhs;
        self.uv *= rhs;
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    triangles: Vec<UVec3>,
    vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            vertices: Vec::new(),
        }
    }

    //getter function?
    pub fn triangles(&self) -> &Vec<UVec3> {
        &self.triangles //kindof functions like index buffer
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices //vertex buffer
    }

    pub fn get_vertices_from_triangle(&self, triangle: UVec3) -> [&Vertex; 3] {
        [
            &self.vertices[triangle.x as usize],
            &self.vertices[triangle.y as usize],
            &self.vertices[triangle.z as usize],
        ] //return triangle as array of vertices
    }

    pub fn from_vertices(triangles: &[UVec3], vertices: &[Vertex]) -> Self {
        let mut mesh = Mesh::new();
        mesh.add_section_from_vertices(triangles, vertices);
        mesh
    }

    pub fn add_section_from_vertices(&mut self, triangles: &[UVec3], vertices: &[Vertex]) {
        let offset = self.vertices.len() as u32;
        let triangles: Vec<UVec3> = triangles.iter().map(|tri| *tri + offset).collect(); //rust closure, iter map
        self.triangles.extend_from_slice(&triangles);
        self.vertices.extend_from_slice(vertices);
    }

    pub fn add_section_from_buffers(
        &mut self,
        triangles: &[UVec3],
        positions: &[Vec3],
        normals: &[Vec3],
        colors: &[Vec3],
        uvs: &[Vec2],
    ) {
        self.triangles.extend_from_slice(triangles);

        let has_uvs = !uvs.is_empty();
        let has_colors = !colors.is_empty();

        for i in 0..positions.len() {
            let vertex = Vertex::Construct(
                positions[i].extend(1.0),
                normals[i],
                if has_colors { colors[i] } else { Vec3::ONE },
                if has_uvs { uvs[i] } else { Vec2::ZERO },
            );
            self.vertices.push(vertex)
        }
    }

    pub fn load_from_gltf(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data]) -> Mesh {
        let mut positions: Vec<Vec3> = Vec::new();
        let mut tex_coords: Vec<Vec2> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut indices = vec![];
        // TODO: handle errors
        let mut result = Mesh::new();
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(indices_reader) = reader.read_indices() {
                indices_reader.into_u32().for_each(|i| indices.push(i));
            }
            if let Some(positions_reader) = reader.read_positions() {
                positions_reader.for_each(|p| positions.push(Vec3::new(p[0], p[1], p[2])));
            }
            if let Some(normals_reader) = reader.read_normals() {
                normals_reader.for_each(|n| normals.push(Vec3::new(n[0], n[1], n[2])));
            }
            if let Some(tex_coord_reader) = reader.read_tex_coords(0) {
                tex_coord_reader
                    .into_f32()
                    .for_each(|tc| tex_coords.push(Vec2::new(tc[0], tc[1])));
            }

            let colors: Vec<Vec3> = positions.iter().map(|_| Vec3::ONE).collect();
            println!("Num indices: {:?}", indices.len());
            println!("tex_coords: {:?}", tex_coords.len());
            println!("positions: {:?}", positions.len());

            let triangles: Vec<UVec3> = indices
                .chunks_exact(3)
                .map(|tri| UVec3::new(tri[0], tri[1], tri[2]))
                .collect();
            result.add_section_from_buffers(&triangles, &positions, &normals, &colors, &tex_coords)
        }
        result
    }
}

//pub fn load_from_gltf() -> Mesh {}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for Mesh {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Self::from_vertices(self.triangles(), self.vertices());
        result.add_section_from_vertices(rhs.triangles(), rhs.vertices());
        result
    }
}

impl AddAssign for Mesh {
    fn add_assign(&mut self, rhs: Self) {
        self.add_section_from_vertices(rhs.triangles(), rhs.vertices());
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    //pub vertices: Vec<Vertex>,
    pub vert0: Vertex,
    pub vert1: Vertex,
    pub vert2: Vertex,
}

pub enum VerticesOrder {
    ABC,
    ACB,
    BAC,
    BCA,
    CAB,
    CBA,
}

impl Triangle {
    pub fn Construct(v0: Vertex, v1: Vertex, v2: Vertex) -> Self //construct because I cling to the past and have trouble letting go...
    {
        Self {
            vert0: v0,
            vert1: v1,
            vert2: v2,
        }
    }

    pub fn transform(&self, matrix: &Mat4) -> Self {
        let p0 = *matrix * self.vert0.position.xyz().extend(1.0);
        let p1 = *matrix * self.vert1.position.xyz().extend(1.0);
        let p2 = *matrix * self.vert2.position.xyz().extend(1.0);

        let mut result = *self;

        result.vert0.position = p0;
        result.vert1.position = p1;
        result.vert2.position = p2;

        result
    }

    pub fn reorder(&self, order: VerticesOrder) -> Self {
        match order {
            VerticesOrder::ABC => *self,
            VerticesOrder::ACB => Self::Construct(self.vert0, self.vert2, self.vert1),
            VerticesOrder::BAC => Self::Construct(self.vert1, self.vert0, self.vert2),
            VerticesOrder::BCA => Self::Construct(self.vert1, self.vert2, self.vert0),
            VerticesOrder::CAB => Self::Construct(self.vert2, self.vert0, self.vert1),
            VerticesOrder::CBA => Self::Construct(self.vert2, self.vert1, self.vert0),
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            vert0: Vertex::default(),
            vert1: Vertex::default(),
            vert2: Vertex::default(),
        }
    }
}
