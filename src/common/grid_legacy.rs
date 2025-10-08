use std::collections::VecDeque;
use std::str::FromStr;

/// Represents a direction in a grid (cardinal or diagonal).
///
/// Directions can be used to navigate in a 2D grid. The cardinal directions
/// include up, down, left, and right, while the diagonal directions include
/// top-left, top-right, bottom-left, and bottom-right.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Direction {
    /// Returns the delta for the direction as a tuple `(x, y)`.
    ///
    /// # Tuple Structure
    /// - `x`: The horizontal movement (column delta). A positive value moves to the right, and a negative value moves to the left.
    /// - `y`: The vertical movement (row delta). A positive value moves downward, and a negative value moves upward.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::Direction::*;
    ///
    /// assert_eq!(Top.as_delta(), (0, -1));        // Move up
    /// assert_eq!(Right.as_delta(), (1, 0));       // Move right
    /// assert_eq!(BottomLeft.as_delta(), (-1, 1)); // Move diagonally bottom-left
    /// ```
    pub fn as_delta(self) -> (isize, isize) {
        match self {
            Direction::Top => (0, -1),
            Direction::TopRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::BottomRight => (1, 1),
            Direction::Bottom => (0, 1),
            Direction::BottomLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::TopLeft => (-1, -1),
        }
    }

    /// Computes the new coordinates by applying the direction's delta to the given position.
    ///
    /// # Parameters
    /// - `col`: A reference to the current column index.
    /// - `row`: A reference to the current row index.
    ///
    /// # Tuple Structure
    /// - `x`: The new column index after applying the direction's horizontal movement.
    /// - `y`: The new row index after applying the direction's vertical movement.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::Direction::*;
    ///
    /// let col = 3;
    /// let row = 3;
    ///
    /// assert_eq!(Top.as_coordinate(&col, &row), (3, 2));        // Move up
    /// assert_eq!(Right.as_coordinate(&col, &row), (4, 3));      // Move right
    /// assert_eq!(BottomLeft.as_coordinate(&col, &row), (2, 4)); // Move diagonally bottom-left
    /// ```
    pub fn as_coordinate(self, col: &usize, row: &usize) -> (isize, isize) {
        (
            *col as isize + self.as_delta().0,
            *row as isize + self.as_delta().1,
        )
    }

    /// Returns an iterator over the cardinal directions: Top, Right, Bottom, Left.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::{Direction, Direction::*};
    ///
    /// let cardinals: Vec<_> = Direction::cardinals().collect();
    /// assert_eq!(cardinals, vec![Top, Right, Bottom, Left]);
    /// ```
    pub fn cardinals() -> impl Iterator<Item = Self> {
        [
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ]
        .iter()
        .copied()
    }

    /// Returns an iterator over the diagonal directions: TopLeft, TopRight, BottomLeft, BottomRight.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::{Direction, Direction::*};
    ///
    /// let diagonals: Vec<_> = Direction::diagonals().collect();
    /// assert_eq!(diagonals, vec![TopRight, BottomRight, BottomLeft, TopLeft]);
    /// ```
    pub fn diagonals() -> impl Iterator<Item = Self> {
        [
            Direction::TopRight,
            Direction::BottomRight,
            Direction::BottomLeft,
            Direction::TopLeft,
        ]
        .iter()
        .copied()
    }

    /// Returns an iterator over all directions (cardinal and diagonal).
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::{Direction, Direction::*};
    ///
    /// let all_directions: Vec<_> = Direction::all().collect();
    /// assert_eq!(
    ///     all_directions,
    ///     vec![Top, TopRight, Right, BottomRight, Bottom, BottomLeft, Left, TopLeft]
    /// );
    /// ```
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Direction::Top,
            Direction::TopRight,
            Direction::Right,
            Direction::BottomRight,
            Direction::Bottom,
            Direction::BottomLeft,
            Direction::Left,
            Direction::TopLeft,
        ]
        .iter()
        .copied()
    }

    /// Returns an iterator over the directions constituting the top-right corner: Top, TopRight, Right.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::{Direction, Direction::*};
    ///
    /// let top_right: Vec<_> = Direction::corner_top_right().collect();
    /// assert_eq!(top_right, vec![Top, TopRight, Right]);
    /// ```
    pub fn corner_top_right() -> impl Iterator<Item = Self> {
        [Direction::Top, Direction::TopRight, Direction::Right]
            .iter()
            .copied()
    }

    /// Returns an iterator over the directions constituting the bottom-right corner: Right, BottomRight, Bottom.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::{Direction, Direction::*};
    ///
    /// let bottom_right: Vec<_> = Direction::corner_bottom_right().collect();
    /// assert_eq!(bottom_right, vec![Right, BottomRight, Bottom]);
    /// ```
    pub fn corner_bottom_right() -> impl Iterator<Item = Self> {
        [Direction::Right, Direction::BottomRight, Direction::Bottom]
            .iter()
            .copied()
    }

    /// Returns an iterator over the directions constituting the bottom-left corner: Bottom, BottomLeft, Left.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::{Direction, Direction::*};
    ///
    /// let bottom_left: Vec<_> = Direction::corner_bottom_left().collect();
    /// assert_eq!(bottom_left, vec![Bottom, BottomLeft, Left]);
    /// ```
    pub fn corner_bottom_left() -> impl Iterator<Item = Self> {
        [Direction::Bottom, Direction::BottomLeft, Direction::Left]
            .iter()
            .copied()
    }

    /// Returns an iterator over the directions constituting the top-left corner: Left, TopLeft, Top.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::{Direction, Direction::*};
    ///
    /// let top_left: Vec<_> = Direction::corner_top_left().collect();
    /// assert_eq!(top_left, vec![Left, TopLeft, Top]);
    /// ```
    pub fn corner_top_left() -> impl Iterator<Item = Self> {
        [Direction::Left, Direction::TopLeft, Direction::Top]
            .iter()
            .copied()
    }
}

/// Represents a 2D grid of elements.
///
/// The `Grid` struct provides utilities to manage a 2D grid of elements, including:
/// - Checking bounds for coordinates.
/// - Finding adjacent elements (cardinal, diagonal, or all).
/// - Creating grids from string input.
#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: VecDeque<VecDeque<T>>,
}

impl<T: Clone> Grid<T> {
    /// Checks if the given coordinates `(col, row)` are within the bounds of the grid.
    ///
    /// # Arguments
    /// - `col`: The column index as an `isize`.
    /// - `row`: The row index as an `isize`.
    ///
    /// # Returns
    /// - `true` if the coordinates are within bounds.
    /// - `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::*;
    ///
    /// let grid: Grid<u32> = Grid::from_delimiter("1 2\n3 4", " ");
    /// assert!(grid.in_bounds(0, 0));
    /// assert!(!grid.in_bounds(10, 10));
    /// ```
    pub fn in_bounds(&self, col: isize, row: isize) -> bool {
        row >= 0 && row < self.rows as isize && col >= 0 && col < self.cols as isize
    }

    /// Finds adjacent cells in the given directions.
    ///
    /// # Arguments
    /// - `col`: The column index of the current cell.
    /// - `row`: The row index of the current cell.
    /// - `directions`: An iterator of directions to check.
    ///
    /// # Returns
    /// A `VecDeque` of valid adjacent cell coordinates `(col, row)`.
    ///
    /// This method is used internally by specific adjacency functions.
    fn check_adjacency(
        &self,
        col: usize,
        row: usize,
        directions: impl Iterator<Item = Direction>,
    ) -> VecDeque<(usize, usize)> {
        directions
            .filter_map(|dir| {
                let nc = col as isize + dir.as_delta().0;
                let nr = row as isize + dir.as_delta().1;

                if self.in_bounds(nc, nr) {
                    Some((nc as usize, nr as usize))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Finds all cardinally adjacent cells (up, down, left, right).
    ///
    /// # Arguments
    /// - `col`: The column index of the current cell.
    /// - `row`: The row index of the current cell.
    ///
    /// # Returns
    /// A `VecDeque` of valid adjacent cell coordinates `(col, row)`.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::*;
    ///
    /// let grid: Grid<u32> = Grid::from_delimiter("1 2\n3 4", " ");
    /// let cardinals = grid.adjacent_cardinals(0, 0);
    /// assert_eq!(cardinals, vec![(1, 0), (0, 1)]);
    /// ```
    pub fn adjacent_cardinals(&self, col: usize, row: usize) -> VecDeque<(usize, usize)> {
        self.check_adjacency(col, row, Direction::cardinals())
    }

    /// Finds all diagonally adjacent cells.
    ///
    /// # Arguments
    /// - `col`: The column index of the current cell.
    /// - `row`: The row index of the current cell.
    ///
    /// # Returns
    /// A `VecDeque` of valid diagonally adjacent cell coordinates `(col, row)`.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::*;
    ///
    /// let grid: Grid<u32> = Grid::from_delimiter("1 2\n3 4", " ");
    /// let diagonals = grid.adjacent_diagonals(0, 0);
    /// assert_eq!(diagonals, vec![(1, 1)]);
    /// ```
    pub fn adjacent_diagonals(&self, col: usize, row: usize) -> VecDeque<(usize, usize)> {
        self.check_adjacency(col, row, Direction::diagonals())
    }

    /// Finds all adjacent cells (cardinal and diagonal).
    ///
    /// # Arguments
    /// - `col`: The column index of the current cell.
    /// - `row`: The row index of the current cell.
    ///
    /// # Returns
    /// A `VecDeque` of valid adjacent cell coordinates `(col, row)`.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::*;
    ///
    /// let grid: Grid<u32> = Grid::from_delimiter("1 2\n3 4", " ");
    /// let all_adjacent = grid.adjacent_eight(0, 0);
    /// assert_eq!(all_adjacent, vec![(1, 0), (1, 1), (0, 1)]);
    /// ```
    pub fn adjacent_eight(&self, col: usize, row: usize) -> VecDeque<(usize, usize)> {
        self.check_adjacency(col, row, Direction::all())
    }

    /// Transposes the grid, swapping rows and columns.
    ///
    /// # Arguments
    ///
    /// This method does not take any arguments.
    ///
    /// # Returns
    ///
    /// This method modifies the grid in place and does not return a value.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::*;
    ///
    /// let mut grid: Grid<u32> = Grid::from_delimiter("1 2 3\n4 5 6", " ");
    /// grid.transpose();
    /// assert_eq!(grid.data, vec![
    ///     vec![1, 4],
    ///     vec![2, 5],
    ///     vec![3, 6],
    /// ]);
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the grid is empty or if the rows have inconsistent lengths.
    ///
    /// # Notes
    ///
    /// - After transposing, the number of rows and columns are swapped.
    /// - The grid must be rectangular (all rows have the same length) for the transpose to work correctly.
    pub fn transpose(&mut self) {
        self.data = (0..self.data[0].len())
            .map(|c| self.data.iter().map(|row| row[c].clone()).collect())
            .collect()
    }
}

impl<T> From<&str> for Grid<T>
where
    T: FromStr + 'static,
    T::Err: std::fmt::Debug,
{
    /// Creates a grid from a string input.
    ///
    /// # Arguments
    /// - `input`: A multi-line string where each line represents a row.
    ///   - Elements within a row are separated by whitespace for numeric types.
    ///   - For character grids, each character is treated as an individual element.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::*;
    ///
    /// let grid: Grid<u32> = Grid::from("12\n34");
    /// assert_eq!(grid.rows, 2);
    /// assert_eq!(grid.cols, 2);
    /// assert_eq!(grid.data[0][0], 1);
    /// assert_eq!(grid.data[1][1], 4);
    ///
    /// let char_grid: Grid<char> = Grid::from(".x\nx.");
    /// assert_eq!(char_grid.rows, 2);
    /// assert_eq!(char_grid.cols, 2);
    /// assert_eq!(char_grid.data[0][0], '.');
    /// assert_eq!(char_grid.data[1][1], '.');
    /// ```
    fn from(input: &str) -> Grid<T> {
        let data: VecDeque<VecDeque<T>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<T>().unwrap())
                    .collect()
            })
            .collect();

        let rows = data.len();
        let cols = data[0].len();

        Grid { rows, cols, data }
    }
}

impl<T> Grid<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    /// Creates a grid from a string input using a specified delimiter.
    ///
    /// # Arguments
    ///
    /// - `input`: A multi-line string where each line represents a row.
    /// - `delimiter`: The delimiter to split each line into elements.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aoc::common::grid_legacy::*;
    ///
    /// let grid = Grid::<u32>::from_delimiter("1 2 3\n4 5 6", " ");
    /// assert_eq!(grid.rows, 2);
    /// assert_eq!(grid.cols, 3);
    /// assert_eq!(grid.data[0][0], 1);
    /// assert_eq!(grid.data[1][2], 6);
    /// ```
    pub fn from_delimiter(input: &str, delimiter: &str) -> Grid<T> {
        let data: VecDeque<VecDeque<T>> = input
            .lines()
            .map(|line| {
                line.split(delimiter)
                    .map(|v| v.parse::<T>().unwrap())
                    .collect()
            })
            .collect();

        let rows = data.len();
        let cols = data[0].len();

        Grid { rows, cols, data }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    /// Formats the grid for display.
    ///
    /// # Example
    /// ```
    /// use aoc::common::grid_legacy::*;
    ///
    /// let grid: Grid<u32> = Grid::from_delimiter("1 2\n3 4", " ");
    /// println!("{}", grid);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.iter() {
            writeln!(
                f,
                "{:?}",
                row.iter().map(ToString::to_string).collect::<Vec<_>>()
            )?;
        }
        Ok(())
    }
}
