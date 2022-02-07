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

        let iterpolated = Lerp(v0, v1, 0.5);
        assert_eq!(interpolated.uv.y, 0.5);
    }
}