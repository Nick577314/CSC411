use std::{iter::Iterator, vec};

 pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    pub vec: Vec<T>,
   
}

impl<T: Clone> Array2<T> {
    // a “blank slate” constructor that allows for a single
    // value to be copied to each element  
   pub fn blank_slate(width: usize, height: usize, val: T) -> Self {
        let data = vec![val.clone(); width * height];
        Array2 {
            width,
            height,
            vec: data,
            
        }
    }
    // getter for width
    pub fn get_width(&self) -> usize {
        self.width
    }

    // getter for height
    pub fn get_height(&self) -> usize {
        self.height
    }
    
    pub fn get(&self, x_cord: usize, y_cord: usize) -> Option<&T> {
        let index = y_cord * self.width + x_cord; // calculates the 

        // Check if the index is within bounds.
        if index < self.vec.len() {
            Some(&self.vec[index]) // reference to the specific element at the designated coordinates
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, x_cord: usize, y_cord: usize) -> Option<&mut T> {
        let index = y_cord * self.width + x_cord;

        // Check if the index is within bounds.
        if index < self.vec.len() {
            Some(&mut self.vec[index]) // reference to the specific element at the designated coordinates
        } else {
            None
        }
    }
    // a function that implements a constructor for row-column major
   pub fn from_row_major(vec: Vec<T>, width: usize, height: usize) -> Array2<T> {
        assert_eq!(width * height, vec.len(), "Invalid input data length");

        Array2 {
            width,
            height,
            vec: vec,
        }
    }

    // a function that implements a constructor for column-row major
    pub fn from_col_major(vec: Vec<T>, width: usize, height: usize) -> Array2<T> {
        assert_eq!(width * height, vec.len(), "Invalid input data length");

        let mut data: Vec<T> = Vec::with_capacity(width * height);

        for col in 0..width {
            // iterates over the columns of the 2D array -> starts at col 0
            for row in 0..height {
                // iterates over the rows of the 2D array -> starts at row 0
                let index = row * width + col; // calculates the index value for the element at that column and row
                data.push(vec[index].clone());
            }
        }
        Array2 {
            width: (width),
            height: (height),
            vec: (data),
        }
    }
    // Lecture 9: -> 10/05/2023 for both iteration functions
    // The Iterator trait is defined in the Rust standard library, and it comes with a default implementation of the next() method.
    // When you return an iterator using impl Iterator,
    // Rust automatically uses this default implementation of next() for your iterator.
    pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.width)
            .flat_map(move |c| (0..self.height).map(move |r| (c, r, self.get(c, r).unwrap())))
    }
    

    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.height)
            .flat_map(move |r| (0..self.width).map(move |c| (c, r, self.get(c, r).unwrap())))


    }
}
