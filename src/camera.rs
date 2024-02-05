use glam::Mat4;

#[derive(Debug)]
pub struct Camera2d {
    pub position: [f32; 2],
    pub width: f32,
    pub height: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera2d {
    pub fn view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_cols_array_2d(
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, -1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        )*glam::Mat4::from_cols_array_2d(
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [-self.position[0], -self.position[1], 0.0, 1.0]
            ])
    }
    
    pub fn projection_matrix(&self) -> glam::Mat4 {
        let left = self.position[0] - 0.5*self.width;
        let right = self.position[0] + 0.5*self.width;
        let top = self.position[1] + 0.5*self.height;
        let bottom = self.position[1] - 0.5*self.height;
        Mat4::orthographic_rh_gl(left, right, bottom, top, self.z_near, self.z_far)
    }
}