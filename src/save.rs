use std::path::Path;
use std::fs::File;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use ::*;

pub fn save<P: AsRef<Path>>(path: P, p: &PhysObj, asts: &[RotatableObj]) {
    let mut file = File::create(path).unwrap();
    bincode::serialize_into(&mut file, &(p, asts), bincode::Infinite).unwrap();
}

pub fn load<P: AsRef<Path>>(path: P, p: &mut PhysObj, asts: &mut Vec<RotatableObj>) {
    let mut file = File::open(path).unwrap();
    let (p_, asts_) = bincode::deserialize_from(&mut file, bincode::Infinite).unwrap();
    *p = p_;
    *asts = asts_;
}

pub fn point_ser<S: Serializer>(p: &Point2, ser: S) -> Result<S::Ok, S::Error> {
    (p.x, p.y).serialize(ser)
}

pub fn vec_ser<S: Serializer>(p: &Vector2, ser: S) -> Result<S::Ok, S::Error> {
    (p.x, p.y).serialize(ser)
}

pub fn point_des<'de, D: Deserializer<'de>>(des: D) -> Result<Point2, D::Error> {
    <(f32, f32)>::deserialize(des).map(|(x, y)| Point2::new(x, y))
}

pub fn vec_des<'de, D: Deserializer<'de>>(des: D) -> Result<Vector2, D::Error> {
    <(f32, f32)>::deserialize(des).map(|(x, y)| Vector2::new(x, y))
}