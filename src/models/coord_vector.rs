use crate::models::coords_2d::Coord2D;
use crate::models::coords_3d::Coord3D;

pub trait MinMax {
    fn min_x(&self) -> i64;
    fn max_x(&self) -> i64;
    fn min_y(&self) -> i64;
    fn max_y(&self) -> i64;
}

impl MinMax for Vec<Coord2D> {
    fn min_x(&self) -> i64 {
        self.iter().map(|p| p.x).min().unwrap()
    }

    fn max_x(&self) -> i64 {
        self.iter().map(|p| p.x).max().unwrap()
    }

    fn min_y(&self) -> i64 {
        self.iter().map(|p| p.y).min().unwrap()
    }

    fn max_y(&self) -> i64 {
        self.iter().map(|p| p.y).max().unwrap()
    }
}

impl MinMax for Vec<Coord3D> {
    fn min_x(&self) -> i64 {
        self.iter().map(|p| p.x).min().unwrap()
    }

    fn max_x(&self) -> i64 {
        self.iter().map(|p| p.x).max().unwrap()
    }

    fn min_y(&self) -> i64 {
        self.iter().map(|p| p.y).min().unwrap()
    }

    fn max_y(&self) -> i64 {
        self.iter().map(|p| p.y).max().unwrap()
    }
}
