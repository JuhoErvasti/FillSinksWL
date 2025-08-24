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

    pub fn length(&self, y_res: f64, x_res: f64) -> f64 {
        match self {
            Direction::North => y_res,
            Direction::East => x_res,
            Direction::South => y_res,
            Direction::West => x_res,
            Direction::NorthEast | Direction::SouthEast | Direction::SouthWest | Direction::NorthWest => (y_res * y_res + x_res * x_res).sqrt(),
        }
    }

    pub fn mindiff(&self, minslope: f64, y_res: f64, x_res: f64) -> f64 {
        self.length(y_res, x_res) * minslope
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

/// Checks if given position is a boundary cell in an array with the given shape. Rows are
/// specified first in both position and shape, i.e. (y, x) and (height, width).
///
/// # Examples
///
/// ```rust
/// use fillsinkswl::fillsinkswl::is_boundary;
///
/// let pos = (99, 200);
/// let shape = (100, 300);
///
/// assert!(is_boundary(pos, shape));
/// ```
///
/// # Panics
///
/// Will panic if width or height won't fit into an i64
pub fn is_boundary(pos: (usize, usize), shape: (usize, usize)) -> bool {
    let (y, x) = pos;
    let (height, width) = shape;


    y == 0 || x == 0 || y == height - 1 || x == width - 1
}

/// Checks if given position is inside an array of given shape). Rows are specified first in both
/// position and shape, i.e. (y, x) and (height, width).
///
/// # Examples
///
/// ```rust
/// use fillsinkswl::fillsinkswl::is_in_array;
///
/// let pos = (-1, 200);
/// let shape = (100, 300);
///
/// assert!(!is_in_array(pos, shape));
/// ```
///
/// # Panics
///
/// Will panic if width or height won't fit into an i64
pub fn is_in_array(pos: (i64, i64), shape: (usize, usize)) -> bool {
    let (y, x) = pos;
    let (height, width) = shape;

    y < height.try_into().unwrap() && x < width.try_into().unwrap() && y >= 0 && x >= 0
}

/// Run Wang & Liu Fill Sinks algorithm. Returns filled raster as a 2D array.
pub fn fill_sinks_wang_liu(
    elevation: &ndarray::Array2<f64>,
    minimum_slope: f64,
    nodata: f64,
    y_res: f64,
    x_res: f64,
) -> ndarray::Array2<f64> {
    let mut minslope = minimum_slope;
    let mut mindiffs= [
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    ];
    if minimum_slope > 0.0 {
        minslope = minslope.to_radians().tan();

        for (i, dir) in Direction::iterator().enumerate() {
            mindiffs[i] = dir.mindiff(minslope, y_res.abs(), x_res.abs());
        }
    }

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

        for (i, dir) in Direction::iterator().enumerate() {
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

            if minslope > 0.0 {
                let mindiff = mindiffs[i];
                if dz < (z + mindiff) {
                    dz = z + mindiff;
                }
            } else if dz < z {
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

#[cfg(test)]
mod test {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_fill_sinks_wang_liu() {
        let input = array![
            [15., 15., 14., 15., 12., 6.,  12.],
            [14., 13., 10., 12., 15., 17., 15.],
            [15., 15., 9.,  11., 8.,  15., 15.],
            [16., 17., 8.,  16., 15., 7.,  5. ],
            [19., 18., 19., 18., 17., 15., 14.],
        ];

        let expected = array![
            [15., 15., 14., 15., 12., 6.0, 12.],
            [14., 13., 11., 12., 15., 17., 15.],
            [15., 15., 11., 11., 8.0, 15., 15.],
            [16., 17., 11., 16., 15., 7.0, 5.0],
            [19., 18., 19., 18., 17., 15., 14.]
        ];

        let output = fill_sinks_wang_liu(&input, 0.0, -9999., 1., 1.);

        assert_eq!(output, expected);
    }
}
