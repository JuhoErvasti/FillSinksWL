use std::slice::Iter;

#[derive(Debug)]
struct Node {
    pub y: i64,
    pub x: i64,
    pub spill: f64,
}

impl Node {
    fn new(y: i64, x: i64, spill: f64) -> Self {
        Self { y, x, spill }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.spill.total_cmp(&self.spill)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.spill.total_cmp(&self.spill))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.spill.total_cmp(&other.spill) == std::cmp::Ordering::Equal
    }
}

impl Eq for Node {
}

#[derive(Debug)]
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

impl Direction {
    pub fn delta(&self, pos: (i64, i64)) -> (i64, i64) {
        let (y, x) = pos;
        match self {
            Direction::North => (y - 1, x),
            Direction::NorthEast => (y - 1, x + 1),
            Direction::East => (y, x + 1),
            Direction::SouthEast => (y + 1, x + 1),
            Direction::South => (y + 1, x),
            Direction::SouthWest => (y + 1, x - 1),
            Direction::West => (y, x - 1),
            Direction::NorthWest => (y - 1, x - 1),
        }
    }

    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ];

        DIRECTIONS.iter()
    }
}

fn is_boundary(pos: (usize, usize), shape: (usize, usize)) -> bool {
    let (y, x) = pos;
    let (height, width) = shape;


    y == 0 || x == 0 || y == height - 1 || x == width - 1
}

fn is_in_array(pos: (i64, i64), shape: (usize, usize)) -> bool {
    let (y, x) = pos;
    let (height, width) = shape;

    y < height.try_into().unwrap() && x < width.try_into().unwrap() && y >= 0 && x >= 0
}

pub fn fill_sinks_wl(
    elevation: &ndarray::Array2<f64>,
    minimum_slope: f64,
    nodata: f64,
) -> ndarray::Array2<f64> {
    let mut queue: std::collections::BinaryHeap<Node> = std::collections::BinaryHeap::new();
    let shape = (elevation.shape()[0], elevation.shape()[1]);

    let mut filled = ndarray::Array2::from_elem(shape, nodata);

    // BOUNDARY CELLS
    for (pos, z) in elevation.indexed_iter() {
        if !is_boundary(pos, shape) && *z != nodata {
            continue;
        }

        let (y, x) = pos;

        queue.push(
            Node::new(
                y as i64,
                x as i64,
                *z,
            )
        );

        filled[[y, x]] = *z;
    }

    // QUEUE
    while !queue.is_empty() {
        // should be fine to unwrap, we just checked if the queue is empty
        let node = queue.pop().unwrap();

        let y = node.y;
        let x = node.x;
        let z = filled[[y as usize, x as usize]];

        for dir in Direction::iterator() {
            let (dy, dx) = dir.delta((y as i64, x as i64));

            if !is_in_array((dy, dx), shape) {
                continue;
            }

            if elevation[[dy as usize, dx as usize]] == nodata {
                continue;
            }

            if filled[[dy as usize, dx as usize]] != nodata {
                continue;
            }

            let mut dz = elevation[[dy as usize, dx as usize]];

            if dz < z {
                dz = z;
            }

            queue.push(
                Node::new(
                    dy,
                    dx,
                    dz,
                )
            );

            filled[[dy as usize, dx as usize]] = dz;
        }
    }

    filled
}

