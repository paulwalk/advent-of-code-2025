use std::cmp::Ordering;

#[derive(Hash,Debug, Clone,Eq, PartialEq)]
pub struct Coord2D {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub struct Coord2DPair {
    pub p: Coord2D,
    pub q: Coord2D,
    pub area: u64,
}

impl Coord2DPair {
    pub fn new(p: Coord2D, q: Coord2D) -> Self {
        let area = area(p.clone(), q.clone());
        Coord2DPair { p, q, area }
    }
}

pub fn compare_area(a: &Coord2DPair, b: &Coord2DPair) -> Ordering {
    if a.area < b.area {
        return Ordering::Less;
    } else if a.area > b.area {
        return Ordering::Greater;
    }
    Ordering::Equal
}

pub fn area(p: Coord2D, q: Coord2D) -> u64 {
    let mut width: i64;
    let mut height: i64;
    if p.x <= q.x {
        width = q.x - p.x;
    } else {
        width = p.x - q.x;
    }
    if p.y <= q.y {
        height = q.y - p.y;
    } else {
        height = p.y - q.y;
    }
    height += 1;
    width += 1;
    let area: u64 = (width * height) as u64;
    area
}