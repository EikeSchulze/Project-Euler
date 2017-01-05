#[derive(Debug, PartialEq)]
pub enum GridError {
    WidthTooSmall,
    GridSizeDoesNotFit,
    ProductLengthTooBig
}

struct Grid<'a> {
    grid: &'a[u64],
    width: usize
}

impl <'a> Grid<'a> {
    pub fn len(&self) -> usize {
        self.grid.len()
    }

    pub fn get(&self, x: usize, y: usize) -> u64 {
        self.grid[x + y * self.width]
    }
}

pub fn largest_product_in_a_grid(grid: &[u64], width: usize, product_length: usize) -> Result<u64, GridError> {
    let grid = Grid{grid: grid, width: width};
    let error = check_for_errors(&grid, product_length);
    match error {
        Some(error) => Err(error),
        None => Ok(safe_calculation(&grid, product_length))
    }
}

fn check_for_errors(grid: &Grid, product_length: usize) -> Option<GridError> {
    if grid.width == 0 {
        return Some(GridError::WidthTooSmall);
    }
    if grid.len() % grid.width != 0 {
        return Some(GridError::GridSizeDoesNotFit);
    }
    if product_length == 0 {
        return None;
    }
    let height = grid.len() / grid.width;
    if product_length > grid.width || product_length > height {
        return Some(GridError::ProductLengthTooBig);
    }
    None
}

fn safe_calculation(grid: &Grid, product_length: usize) -> u64 {
    if product_length == 0 {
        return 1;
    }
    if product_length == 1 {
        let &result = grid.grid.into_iter().max().unwrap_or(&1);
        return result;
    }
    let height = grid.len() / grid.width;

    //TODO: do the actual product in horizontal, vertical, diagonal, anti-diagonal
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_product_in_a_grid_width_too_small() {
        assert_eq!(Err(GridError::WidthTooSmall), largest_product_in_a_grid(&[1], 0, 1));
    }

    #[test]
    fn test_largest_product_in_a_grid_grid_size_does_not_fit() {
        assert_eq!(Err(GridError::GridSizeDoesNotFit), largest_product_in_a_grid(&[1], 10, 1));
    }

    #[test]
    fn test_largest_product_in_a_grid_product_length_zero() {
        assert_eq!(Ok(1), largest_product_in_a_grid(&[15], 1, 0));
    }

    #[test]
    fn test_largest_product_in_a_grid_product_length_too_big() {
        assert_eq!(Err(GridError::ProductLengthTooBig), largest_product_in_a_grid(&[1], 1, 10));
    }

    #[test]
    fn test_largest_product_in_a_grid() {
        assert_eq!(Ok(1), largest_product_in_a_grid(&[1], 1, 1));
        assert_eq!(Ok(5), largest_product_in_a_grid(&[5], 1, 1));
        assert_eq!(Ok(10), largest_product_in_a_grid(&[2, 1, 0, 5], 2, 2));
        assert_eq!(Ok(5), largest_product_in_a_grid(&[2, 1, 0, 5], 2, 1));
    }

    #[test]
    fn test_largest_product_in_a_grid_product_length_1() {
        assert_eq!(Ok(8), largest_product_in_a_grid(&[1, 2, 3, 4, 5, 6, 7, 8], 2, 1));
        assert_eq!(Ok(9), largest_product_in_a_grid(&[5, 1, 4, 2, 6, 9], 3, 1));
    }
}