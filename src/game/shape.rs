pub mod vector2;
use vector2::Vector2;

// We only need two kinds of shape: circles and polygons
trait Shape {
    fn is_colliding(p1: &Vector2, p2: &Vector2, that: &Shape) -> bool;
}

pub struct Circle {
    radius: f32
}

pub struct Polygon {
    points: Vec<Vector2>
}

impl Shape for Circle {

}
