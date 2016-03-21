pub struct BulletType {
    asset_id: String,
    asset_quad: [u32, 4],
}

trait Collidable {
    fn is_colliding(that: Collidable) -> bool;
}
