use matrix::Matrix;

mod matrix;

// Example usage
fn main() {
    // Create a new matrix
    let mut matrix1 = Matrix::from_file("matrix.txt").unwrap();

    // Zoom the matrix
    matrix1.zoom(2.0);

    println!("{}", matrix1);

}
