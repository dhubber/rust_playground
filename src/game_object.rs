#[derive(Copy, Clone, Debug)]
pub struct GameObject {
    pub id: u128,
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub color: [f32; 4]
}