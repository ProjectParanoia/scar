use cgmath::Point2;
pub enum Shape {
    Circle{radius: f32},
    Polygon{points: Vec<Point2>}
}
