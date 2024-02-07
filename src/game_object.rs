#[derive(Copy, Clone, Debug)]
pub struct Transform2d {
    pub position: [f32; 2],
    pub scale: [f32; 2],
}

#[derive(Copy, Clone, Debug)]
pub struct Renderable {
    pub color: [f32; 4]
}

#[derive(Copy, Clone, Debug)]
pub struct GameObject {
    pub id: u128,
    pub transform2d: Transform2d,
    pub renderable: Renderable,
}