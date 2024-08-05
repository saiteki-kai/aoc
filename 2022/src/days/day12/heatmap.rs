#[derive(Clone)]
pub struct HeatMap {
    n_rows: usize,
    n_cols: usize,
    squares: Vec<Square>,
    index: usize,
    start_idx: usize,
}

impl HeatMap {
    pub fn new(squares: Vec<Square>, rows: usize, cols: usize, start_idx: usize) -> Self {
        HeatMap {
            squares,
            n_rows: rows,
            n_cols: cols,
            index: 0,
            start_idx,
        }
    }

    pub fn rows(&self) -> usize {
        self.n_rows
    }

    pub fn cols(&self) -> usize {
        self.n_cols
    }

    pub fn len(&self) -> usize {
        self.squares.len()
    }

    pub fn start(&self) -> Option<&Square> {
        self.squares.get(self.start_idx)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&Square> {
        let index = row * self.n_cols + col;

        self.squares.get(index)
    }

    pub fn neighbours(&self, square: &Square) -> Vec<Square> {
        let positions: Vec<(usize, usize)> = vec![
            (Some(square.row), square.col.checked_sub(1)),
            (Some(square.row), square.col.checked_add(1)),
            (square.row.checked_sub(1), Some(square.col)),
            (square.row.checked_add(1), Some(square.col)),
        ]
        .into_iter()
        .filter_map(|(a, b)| {
            if let (Some(a), Some(b)) = (a, b) {
                Some((a, b))
            } else {
                None
            }
        })
        .collect();

        positions
            .into_iter()
            .filter_map(|(row, col)| self.get(row, col))
            .cloned()
            .collect()
    }

    pub fn possible_destinations(&self, square: &Square) -> Vec<Square> {
        self.neighbours(square)
            .iter()
            .filter(|s| {
                let level_diff = s.get_elevation().expect("bad value") as i8
                    - square.get_elevation().expect("bad value") as i8;

                // next level || same level
                level_diff == 1 || level_diff == 0
            })
            .cloned()
            .collect()
    }
}

impl Iterator for HeatMap {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.squares.len() {
            let square = self.squares[self.index];
            self.index += 1;

            Some(square)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub value: char,
    pub row: usize,
    pub col: usize,
    // up: Option<Rc<RefCell<Square>>>,
    // down: Option<Rc<RefCell<Square>>>,
    // right: Option<Rc<RefCell<Square>>>,
    // left: Option<Rc<RefCell<Square>>>,
}

impl Ord for Square {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_elevation().cmp(&other.get_elevation())
    }
}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Square {
    pub fn new(row: usize, col: usize, elevation: char) -> Self {
        Self {
            value: elevation,
            row,
            col,
        }
    }

    pub fn get_elevation(&self) -> Option<u8> {
        match self.value {
            'S' => Some(0),               // start
            'E' => Some(b'z' - b'a' + 1), // end
            'a'..='z' => Some(self.value as u8 - b'a' + 1),
            _ => None, // error
        }
    }

    pub fn is_start(&self) -> bool {
        self.value.eq(&'S')
    }

    pub fn is_end(&self) -> bool {
        self.value.eq(&'E')
    }
}

fn map_line(row_index: usize, line: &str) -> Vec<Square> {
    line.chars()
        .enumerate()
        .map(|(col_index, char)| Square::new(row_index, col_index, char))
        .collect()
}

pub fn read_heatmap(input: &str) -> HeatMap {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();

    let n_rows = rows.len();
    let n_cols = rows.first().unwrap().len(); // TODO: remove unwrap

    let squares: Vec<Square> = rows
        .iter()
        .enumerate()
        .flat_map(|(i, s)| map_line(i, s))
        .collect();

    let (start_idx, _) = squares
        .iter()
        .enumerate()
        .find(|(_, s)| s.is_start())
        .expect("Start position not defined");

    HeatMap::new(squares, n_rows, n_cols, start_idx)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::Square;

    #[test_case(Square::new(0, 0, 'S'), true)]
    #[test_case(Square::new(0, 0, 'E'), true)]
    #[test_case(Square::new(0, 0, 'z'), true)]
    #[test_case(Square::new(0, 0, 'X'), false)]
    #[test_case(Square::new(0, 0, '3'), false)]
    #[test_case(Square::new(0, 0, 'ç'), false)]
    fn test_square_elevation(square: Square, expect: bool) {
        assert_eq!(square.get_elevation().is_some(), expect);
    }
}
