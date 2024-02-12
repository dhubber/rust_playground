#[derive(Copy, Clone, Debug)]
pub struct AxisAlignedBox {
    pub min: [f32; 2],
    pub max: [f32; 2]
}

impl PartialEq<Self> for AxisAlignedBox {
    fn eq(&self, other: &Self) -> bool {
        self.min.eq(&other.min) && self.max.eq(&other.max)
    }
}

impl AxisAlignedBox {
    pub fn centre(&self) -> [f32; 2] {
        [0.5*(self.min[0] + self.max[0]), 0.5*(self.min[1] + self.max[1])]
    }

    pub fn is_overlapping(aab1: &AxisAlignedBox, aab2: &AxisAlignedBox) -> bool {
        if aab1.min[0] > aab2.max[0] { return false; }
        if aab2.min[0] > aab1.max[0] { return false; }
        if aab1.min[1] > aab2.max[1] { return false; }
        if aab2.min[1] > aab1.max[1] { return false; }
        true
    }
}