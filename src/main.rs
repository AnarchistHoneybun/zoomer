use matrix::Matrix;

mod matrix;

// Example usage
fn main() {
    // Create a new matrix
    let mut matrix = Matrix::from_file("matrix.txt").unwrap();

    // Zoom the matrix
    matrix.zoom(3.0);

    println!("{}", matrix);
}
