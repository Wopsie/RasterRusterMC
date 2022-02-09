use glam::{Vec2, Vec3, UVec3};
use std::{ops::{Add, Mul, Sub, MulAssign, AddAssign}, f32::MIN_POSITIVE};

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
    pub position: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}

//implementation, bind functions to Vertex data struct
impl Vertex 
{
    //return self, like a constructor kindof
    pub fn Construct(position: Vec3, color: Vec3, uv: Vec2) -> Self{
        Self {
            position,
            color,
            uv,
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
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
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self::Construct(position, color, uv)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self{
        let position = self.position - rhs.position;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self::Construct(position, color, uv)
    }
}

impl Mul<f32> for Vertex{
    type Output = Self;
    
    fn mul(self, rhs: f32) -> Self{
        let position = self.position * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self::Construct(position, color, uv)
    }
}

impl MulAssign<f32> for Vertex {
    fn mul_assign(&mut self, rhs: f32) {
        self.position *= rhs;
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
        &self.vertices  //vertex buffer
    }
    
    pub fn get_vertices_from_triangle(&self, triangle: UVec3) -> [&Vertex; 3] {
        [
            &self.vertices[triangle.x as usize],
            &self.vertices[triangle.y as usize],
            &self.vertices[triangle.z as usize],
        ]   //return triangle as array of vertices
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
}

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

impl Triangle 
{
    pub fn Construct(v0: Vertex, v1: Vertex, v2: Vertex) -> Self
    {
        Self{
            vert0: v0,
            vert1: v1,
            vert2: v2,
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
