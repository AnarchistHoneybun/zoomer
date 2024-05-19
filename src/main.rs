use matrix::Matrix;
use std::io::BufRead;

mod matrix;

// Example usage
fn main() {
    // Create a new matrix
    let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let mut matrix = Matrix::new(data[0].len(), data.len(), Some(data));

    // Zoom the matrix by a factor of 2
    matrix.zoom(5.0);

    println!("{}", matrix);
}
