use glam::{Vec2, Vec3};
use std::{ops::{Add, Mul, Sub}, f32::MIN_POSITIVE};

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
        let texCoord = self.uv + rhs.uv;
        Self::Construct(position, color, texCoord)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self{
        let position = self.position - rhs.position;
        let color = self.color - rhs.color;
        let texCoord = self.uv - rhs.uv;
        Self::Construct(position, color, texCoord)
    }
}

impl Mul for Vertex{
    type Output = Self;
    
    fn mul(self, rhs:Self) -> Self{
        let position = self.position * rhs.position;
        let color = self.color * rhs.color;
        let texCoord = self.uv * rhs.uv;
        Self::Construct(position, color, texCoord)
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

pub struct Point{
    pub x: i32,
    pub y: i32,
}