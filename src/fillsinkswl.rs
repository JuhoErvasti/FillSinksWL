struct Node {
    x: i64,
    y: i64,
    spill: f64,
}

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub struct FillSinks {
    array: ndarray::Array2<f64>,
}

impl FillSinks {
    pub fn new(array: ndarray::Array2<f64>) -> Self {
        Self { array }
    }

    pub fn execute() {
        todo!()
    }

    fn dir(x: i64, y: i64, z: f64) -> Direction {
        todo!()
    }
}


