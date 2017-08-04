
use std::fmt;

pub struct DenseMatrix {
    ns: Vec<f64>,
    pub row_count: usize,
    pub col_count: usize,
}

impl DenseMatrix {

    /// Constructs from a grid of fixed-size rows separated by newlines
    /// and values in each row separated by spaces
    pub fn from_grid(grid: &str) -> Option<DenseMatrix> {

        let mut row_count = 0;
        let mut consistent_col_count = 0;
        let mut inconsistent_row_size = false;
        let mut ns: Vec<f64> = Vec::new();

        for line in grid.lines() {
            let mut col_count = 0;
            for entry in line.split_whitespace() {
                match entry.parse() {
                    Ok(n) => {
                        ns.push(n);
                        col_count += 1;
                    },
                    Err(_) => {}
                }
            }
            row_count += 1;
            if consistent_col_count != 0 && consistent_col_count != col_count {
                inconsistent_row_size = true;
            }
            consistent_col_count = col_count;
        }

        if !inconsistent_row_size {
            Some(DenseMatrix { ns: ns, row_count: row_count, col_count: consistent_col_count })
        } else {
            None
        }
    }
}

impl DenseMatrix {

    /// Rows of the matrix
    fn rows<'a>(&'a self) -> DenseMatrixRows<'a> {
        DenseMatrixRows { m: self, next_index: 0 }
    }

    /// All the left-leaning diagonal segments of the given length in the specified square matrix
    pub fn diagonal_segments_left<'a>(&'a self, length: usize) -> DenseMatrixDiagonalSegmentsLeft<'a> {
        DenseMatrixDiagonalSegmentsLeft { m: self, next_index: (0, 0), length: length }
    }

    /// All the horizontal segments of the given length in the specified square matrix
    pub fn horizontal_segments<'a>(&'a self, length: usize) -> DenseMatrixHorizontalSegments<'a> {
        DenseMatrixHorizontalSegments { m: self, next_index: (0, 0), length: length }
    }

    /// All the vertical segments of the given length in the specified square matrix
    pub fn vertical_segments<'a>(&'a self, length: usize) -> DenseMatrixVerticalSegments<'a> {
        DenseMatrixVerticalSegments { m: self, next_index: (0, 0), length: length }
    }

    /// All the right-leaning diagonal segments of the given length in the specified square matrix
    pub fn diagonal_segments_right<'a>(&'a self, length: usize) -> DenseMatrixDiagonalSegmentsRight<'a> {
        DenseMatrixDiagonalSegmentsRight { m: self, next_index: (0, self.col_count-1), length: length }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row < self.row_count && col < self.col_count {
            match self.ns.get(row*self.col_count+col) {
                Some(&n) => Some(n),
                None => None,
            }
        } else {
            None
        }
    }
}

struct DenseMatrixRows<'a> {
    m: &'a DenseMatrix,
    next_index: usize
}

impl<'a> Iterator for DenseMatrixRows<'a> {
    type Item = &'a [f64];

    fn next(&mut self) -> Option<&'a [f64]> {
        if self.next_index >= (self.m.ns.len()) {
            None
        } else {
            let row: &'a [f64] = &self.m.ns[self.next_index..(self.next_index+self.m.col_count)];
            self.next_index += self.m.col_count;
            Some(row)
        }
    }
}

struct DenseMatrixDiagonalSegmentsLeft<'a> {
    m: &'a DenseMatrix,
    next_index: (usize, usize),
    length: usize,
}

impl <'a> Iterator for DenseMatrixDiagonalSegmentsLeft<'a> {
    type Item = Vec<f64>;

    fn next(&mut self) -> Option<Vec<f64>> {
        let (next_row, next_col) = self.next_index;
        if (next_row+self.length) <= self.m.row_count && (next_col+self.length) <= self.m.col_count {
            self.next_index = (next_row, next_col+1);
            let mut result = Vec::with_capacity(self.length);
            for (i, j) in (next_row..next_row+self.length).zip(next_col..next_col+self.length) {
                result.push(self.m.get(i, j).expect("Failed to find entry in diagonal left segment"))
            }
            Some(result)
        } else if (next_row+self.length) <= self.m.row_count && (next_col+self.length) > self.m.col_count {
            self.next_index = (next_row+1, 0);
            self.next()
        } else {
            None
        }
    }
}

struct DenseMatrixDiagonalSegmentsRight<'a> {
    m: &'a DenseMatrix,
    next_index: (usize, usize),
    length: usize,
}

impl<'a> Iterator for DenseMatrixDiagonalSegmentsRight<'a> {
    type Item = Vec<f64>;

    fn next(&mut self) -> Option<Vec<f64>> {
        let (next_row, next_col) = self.next_index;
        if (next_row+self.length) <= self.m.row_count && (next_col+1) >= self.length {
            self.next_index = (next_row, next_col-1);
            let mut result = Vec::with_capacity(self.length);
            for (row, col) in (next_row..next_row+self.length).zip((next_col+1-self.length..next_col+1).rev()) {
                result.push(self.m.get(row, col).expect("Failed to find entry in diagonal right segment"))
            }
            Some(result)
        } else if (next_row+self.length) <= self.m.row_count && next_col < self.length {
            self.next_index = (next_row+1, self.m.col_count-1);
            self.next()
        } else {
            None
        }
    }
}

struct DenseMatrixHorizontalSegments<'a> {
    m: &'a DenseMatrix,
    next_index: (usize, usize),
    length: usize,
}

struct DenseMatrixVerticalSegments<'a> {
    m: &'a DenseMatrix,
    next_index: (usize, usize),
    length: usize,
}

impl <'a> Iterator for DenseMatrixHorizontalSegments<'a> {
    type Item = Vec<f64>;
    fn next(&mut self) -> Option<Vec<f64>> {
        let (next_row, next_col) = self.next_index;
        if next_row < self.m.row_count && (next_col+self.length) <= self.m.col_count {
            self.next_index = (next_row, next_col+1);
            let mut result = Vec::with_capacity(self.length);
            for j in next_col..next_col+self.length {
                result.push(self.m.get(next_row, j).expect("Failed to find an entry in a horizontal segment"))
            }
            Some(result)
        } else if next_row < self.m.row_count && (next_col+self.length) > self.m.col_count {
            self.next_index = (next_row+1, 0);
            self.next()
        } else {
            None
        }
    }
}

impl <'a> Iterator for DenseMatrixVerticalSegments<'a> {
    type Item = Vec<f64>;
    fn next(&mut self) -> Option<Vec<f64>> {
        let (next_row, next_col) = self.next_index;
        if (next_row+self.length) <= self.m.row_count && next_col < self.m.col_count {
            self.next_index = (next_row, next_col+1);
            let mut result = Vec::with_capacity(self.length);
            for i in next_row..next_row+self.length {
                result.push(self.m.get(i, next_col).expect("Failed to find an entry in a vertical segment"))
            }
            Some(result)
        } else if (next_row+self.length) <= self.m.row_count && next_col >= self.m.col_count {
            self.next_index = (next_row+1, 0);
            self.next()
        } else {
            None
        }
    }
}

impl fmt::Display for DenseMatrix {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows() {
            for n in row {
                match write!(f, "{} ", n) {
                    Ok(_) => (),
                    Err(err) => return Err(err)
                }
            }
            match writeln!(f, "{}", "") {
                Ok(_) => (),
                Err(err) => return Err(err)
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn correct_horizontal_segments() {
        let matrix = DenseMatrix::from_grid("1  2  3  4
                                             7  8  9  10
                                             25 26 27 28").expect("Failed to parse grid matrix");

        let segments: Vec<Vec<u32>> = matrix.horizontal_segments(2).map(|v| v.iter().map(|&n| n as u32).collect()).collect();

        assert_eq!(segments, vec![
            vec![1, 2], vec![2, 3], vec![3, 4], vec![7, 8], vec![8, 9], vec![9, 10], vec![25, 26], vec![26, 27], vec![27, 28]
        ]);
    }

    #[test]
    fn correct_vertical_segments() {
        let matrix = DenseMatrix::from_grid("1  2  3  4
                                             7  8  9  10
                                             25 26 27 28").expect("Failed to parse grid matrix");

        let segments: Vec<Vec<u32>> = matrix.vertical_segments(2).map(|v| v.iter().map(|&n| n as u32).collect()).collect();

        assert_eq!(segments, vec![
            vec![1, 7], vec![2, 8], vec![3, 9], vec![4, 10], vec![7, 25], vec![8, 26], vec![9, 27], vec![10, 28]
        ]);
    }

    #[test]
    fn correct_diagonal_left_segments() {
        let matrix = DenseMatrix::from_grid("1  2  3  4
                                             7  8  9  10
                                             25 26 27 28").expect("Failed to parse grid matrix");

        let segments: Vec<Vec<u32>> = matrix.diagonal_segments_left(2).map(|v| v.iter().map(|&n| n as u32).collect()).collect();

        assert_eq!(segments, vec![
            vec![1, 8], vec![2, 9], vec![3, 10], vec![7, 26], vec![8, 27], vec![9, 28]
        ]);
    }

    #[test]
    fn correct_diagonal_right_segments() {
        let matrix = DenseMatrix::from_grid("1  2  3  4
                                             7  8  9  10
                                             25 26 27 28").expect("Failed to parse grid matrix");

        let segments: Vec<Vec<u32>> = matrix.diagonal_segments_right(2).map(|v| v.iter().map(|&n| n as u32).collect()).collect();

        assert_eq!(segments, vec![
            vec![4, 9], vec![3, 8], vec![2, 7], vec![10, 27], vec![9, 26], vec![8, 25]
        ]);
    }
}
