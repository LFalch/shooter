use std::path::Path;
use std::fs::File;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use ::*;

/// Save the state in a file
pub fn save<P: AsRef<Path>>(path: P, w: &World) {
    let mut file = File::create(path).unwrap();
    bincode::serialize_into(&mut file, w, bincode::Infinite).unwrap();
}
/// Load the state from a file
pub fn load<P: AsRef<Path>>(path: P, w: &mut World) {
    let mut file = File::open(path).unwrap();
    let world = bincode::deserialize_from(&mut file, bincode::Infinite).unwrap();
    *w = world;
}
/// Serialize a `Point2`
pub fn point_ser<S: Serializer>(p: &Point2, ser: S) -> Result<S::Ok, S::Error> {
    (p.x, p.y).serialize(ser)
}
/// Serialize a `Vector2`
pub fn vec_ser<S: Serializer>(p: &Vector2, ser: S) -> Result<S::Ok, S::Error> {
    (p.x, p.y).serialize(ser)
}
/// Deserialize a `Point2`
pub fn point_des<'de, D: Deserializer<'de>>(des: D) -> Result<Point2, D::Error> {
    <(f32, f32)>::deserialize(des).map(|(x, y)| Point2::new(x, y))
}
/// Deserialize a `Vector2`
pub fn vec_des<'de, D: Deserializer<'de>>(des: D) -> Result<Vector2, D::Error> {
    <(f32, f32)>::deserialize(des).map(|(x, y)| Vector2::new(x, y))
}
