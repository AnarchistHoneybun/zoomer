use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Define the Matrix struct
#[derive(Debug)]
pub struct Matrix {
    dim_x: usize,
    dim_y: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    // Constructor function
    pub fn new(dim_x: usize, dim_y: usize, data: Option<Vec<Vec<f64>>>) -> Matrix {
        let mut matrix_data = Vec::with_capacity(dim_y);
        for _ in 0..dim_y {
            let row = Vec::with_capacity(dim_x);
            matrix_data.push(row);
        }

        if let Some(data) = data {
            if data.len() != dim_y || data.iter().any(|row| row.len() != dim_x) {
                panic!("Invalid data dimensions");
            }
            matrix_data = data;
        }

        Matrix {
            dim_x,
            dim_y,
            data: matrix_data,
        }
    }

    // Function to read a matrix from a plaintext file
    pub fn from_file(file_path: &str) -> Result<Matrix, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut data = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let row: Vec<f64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            data.push(row);
        }

        let dim_y = data.len();
        let dim_x = data[0].len();

        Ok(Matrix { dim_x, dim_y, data })
    }

    // Function to zoom or shrink the matrix
    pub fn zoom(&mut self, zoom_factor: f64) {
        let new_dim_x = (self.dim_x as f64 * zoom_factor) as usize;
        let new_dim_y = (self.dim_y as f64 * zoom_factor) as usize;

        let mut new_data = Vec::with_capacity(new_dim_y);
        for _ in 0..new_dim_y {
            let row = vec![0.0; new_dim_x];
            new_data.push(row);
        }

        for y in 0..self.dim_y {
            for x in 0..self.dim_x {
                let new_x = (x as f64 * zoom_factor) as usize;
                let new_y = (y as f64 * zoom_factor) as usize;
                new_data[new_y][new_x] = self.data[y][x];
            }
        }

        self.dim_x = new_dim_x;
        self.dim_y = new_dim_y;
        self.data = new_data;
    }
}

// Implement the Display trait for Matrix
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for value in row {
                write!(f, "{} ", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
