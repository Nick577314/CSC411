use array2::Array2;
use csc411_image::{Gray, GrayImage, Read};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = if args.len() > 1 {
        // if there are more than 1 argument then accept command line
        Some(args[1].as_str()) // Convert String to &str
    } else {
        // else pass None to Read
        None
    };
    // since None was passed to Read then it will go to standard input
    let gray_image = GrayImage::read(input_file).unwrap_or_else(|err| {
        eprintln!("Error reading the image: {}", err);
        exit(1);
    });

    let gray_data: Vec<Gray> = gray_image.pixels.clone(); //returns a copy of the pixel values
    let gray_width: usize = gray_image.width as usize;
    let gray_height: usize = gray_image.height as usize;

    //create grid1 using from_row_major constructor from our Array2 struct
    // passing the gray_data variable as the vector
    let grid1 = Array2::from_row_major(gray_data, gray_width, gray_height);

    if sudoku_validator(&grid1) {
        exit(0);
    } else {
        exit(1);
    }
}

fn sudoku_validator(grid: &Array2<Gray>) -> bool {
    if grid.get_width() != 9 || grid.get_height() != 9 {
        return false;
    }
    // iterates through the grid in row major order
    // the (_,_ , pixel) it ignores the width and height
    for (_, _, pixel) in grid.iter_row_major() {
        // checks if the pixel values are within the appropriate range of 1 - 9 if not return false
        if pixel.value < 1 || pixel.value > 9 {
            return false;
        }
    }
    // iterates through the grid in row major order
    for (_, _, pixel) in grid.iter_row_major() {
        // Create a vector of type bool to keep track of values in the current row.
        let mut row_set = vec![false; 10];

        if row_set[pixel.value as usize] {
            // if the value has already been seen it will return false
            return false;
        }
        row_set[pixel.value as usize] = true;
    }
    // iterates through the grid in column major order
    for (_, _, pixel) in grid.iter_col_major() {
        // Create a vector of type bool to keep track of values in the current column.
        let mut col_set = vec![false; 10];

        if col_set[pixel.value as usize] {
            // if the value has already been seen it will return false
            return false;
        }

        col_set[pixel.value as usize] = true;
    }

    // iterates through 3x3 subgrids of the larger grid
    for i in 0..3 {
        for j in 0..3 {
            // Create a vector of type bool to keep track of values in the current subgrid.
            let mut subgrid_set = vec![false; 10];

            // iterates through the cells of the subgrid
            for x in 0..3 {
                for y in 0..3 {
                    // this uses the get method we created to return a specific pixel value at specific cordinates
                    let pixel = grid.get(i * 3 + x, j * 3 + y).unwrap();

                    if subgrid_set[pixel.value as usize] {
                        // if the value has already been seen it will return false
                        return false;
                    }
                    subgrid_set[pixel.value as usize] = true;
                }
            }
        }
    }
    true
}
