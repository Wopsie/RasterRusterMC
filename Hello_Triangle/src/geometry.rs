use glam::{Vec2, Vec3};
use std::{ops::{Add, Mul, Sub}, f32::MIN_POSITIVE};

//data struct
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
    pub texCoord: Vec2,
}

//implementation, bind functions to Vertex data struct
impl Vertex 
{
    //return self, like a constructor kindof
    pub fn Construct(position: Vec3, color: Vec3, texCoord: Vec2) -> Self{
        Self {
            position,
            color,
            texCoord,
        }
    }
}

//implements Add trait for vertex. Implementation is custom, but Add trait is generic. Traits are used to identify if generic objects implement certain functionality.
impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let position = self.position + rhs.position;
        let color = self.color + rhs.color;
        let texCoord = self.texCoord + rhs.texCoord;
        Self::Construct(position, color, texCoord)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self{
        let position = self.position - rhs.position;
        let color = self.color - rhs.color;
        let texCoord = self.texCoord - rhs.texCoord;
        Self::Construct(position, color, texCoord)
    }
}

impl Mul for Vertex{
    type Output = Self;
    
    fn mul(self, rhs:Self) -> Self{
        let position = self.position * rhs.position;
        let color = self.color * rhs.color;
        let texCoord = self.texCoord * rhs.texCoord;
        Self::Construct(position, color, texCoord)
    }
}

pub struct Triangle {
    //pub vertices: Vec<Vertex>,
    pub vert0: Vertex,
    pub vert1: Vertex,
    pub vert2: Vertex,
}