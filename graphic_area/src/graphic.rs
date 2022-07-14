pub trait AreaCalculable {
    fn calculate_area(self) -> f32;
}

pub struct Circle {
    pub r: f32,
}

impl AreaCalculable for Circle {
    // 圆形面积: 3.14 * r * r
    fn calculate_area(self) -> f32 {
        return 3.1415926 * self.r * self.r;
    }
}

pub struct Triangle {
    pub a: f32,
    pub h: f32,
}

impl AreaCalculable for Triangle {
    // 三角形: 1/2 * 底 * 高
    fn calculate_area(self) -> f32 {
        return 1.0 / 2.0 * self.a * self.h;
    }
}

pub struct Square {
    pub a: f32,
}

impl AreaCalculable for Square {
    // 正方形: 边长 * 边长
    fn calculate_area(self) -> f32 {
        return self.a * self.a;
    }
}
