use std::cmp::Ordering;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Coord3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Coord3DPair {
    pub p: Coord3D,
    pub q: Coord3D,
}

impl Coord3DPair {
    pub fn euclidian_distance(&self) -> f64 {
        let dx = (self.p.x - self.q.x) as f64;
        let dy = (self.p.y - self.q.y) as f64;
        let dz = (self.p.z - self.q.z) as f64;
        ((dx * dx) + (dy * dy) + (dz * dz)).sqrt()
    }
}

pub fn compare_distance(a: &Coord3DPair, b: &Coord3DPair) -> Ordering {
    if a.euclidian_distance() < b.euclidian_distance() {
        return Ordering::Less;
    } else if a.euclidian_distance() > b.euclidian_distance() {
        return Ordering::Greater;
    }
    Ordering::Equal
}
