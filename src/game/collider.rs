use std::tuple;
use cgmath::Point2;
use game::shape::Shape::{Circle, Polygon};

pub fn is_collide(s1: Shape, p1: (), s2: Shape, v2: Vector2) -> bool {
    match (s1, s2)
}

fn is_collide_polygon_circle(p: Polygon, pos1: (i32, i32), c: Circle, pos2: (i32, i32)) -> bool {

}

fn is_collide_circle_circle(c1: &Circle, pos1: &Point2, c2: &Circle, pos2: &Point2) -> bool {
    let (r1, r2) = (c1.radius, c2.radius);
    return (pos1 - pos2).length() <= r1 + r2
}

#[test]
fn circle_circle_collide() {
    assert!(is_collide())
}
