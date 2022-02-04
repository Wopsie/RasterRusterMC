use glam::{Vec2, Vec3};

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

pub fn bresenham_function(vert0: Vec2, vert1: Vec2, pixel: Vec2) -> f32
{
    //let newSlope = 2.0 * (v1.y - v0.y);
    //let slopeErrorNew = newSlope - (v1.x - v0.x);

    let deltax = vert1.x - vert0.x;
    let deltay = vert1.y - vert0.y;

    let twoDeltaY = 2.0 * deltay;
    let twoDyDx = 2.0 * (deltay - deltax);

    let error = deltax * 0.5;
    let ystep = 1;

    // let xPos = 0.0;
    // let yPos = 0.0;

    if vert0.x > vert1.x
    {
        // xPos = vert1.x;
        // yPos = vert1.y;
        0.0
    }else {
        1.0
    }


    // if vert1.y < vert0.y
    // {
    //     ystep = -1;
    // }

    // int dx = abs(xa - xb), dy = abs(ya - yb);
    // int p = 2 * dy - dx;
    // int twoDy = 2 * dy, twoDyDx = 2 * (dy - dx);
    // int x, y, xEnd;

    // if (xa > xb)
    // {
    //     x = xb;
    //     y = yb;
    //     xEnd = xa;
    // }
    // else
    // {
    //     x = xa;
    //     y = ya;
    //     xEnd = xb;
    // }
    // setPixel(x, y);

    //loop
    

    //(((v1.y - v0.y) / (v1.x - v0.x)) * (p.x - v0.x) + v0.y)
}
