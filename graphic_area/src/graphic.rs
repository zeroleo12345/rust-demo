// 圆形面积: 3.14 * r * r
// 三角形: 1/2 * 底 * 高
// 正方形: 边长 * 边长

pub trait AreaCalculable {
    fn calculate_area(self) -> f32;
}

pub struct Circle {
    pub r: f32,
}

impl AreaCalculable for Circle {
    fn calculate_area(self) -> f32 {
        return 3.1415926 * self.r * self.r;
    }
}
