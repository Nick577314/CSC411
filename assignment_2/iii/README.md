# Array2 and Sudoku checker
----
`by Nicolas Leffray` 

- I Discussed the design of the array2 with Aidan Kelly 
- The code should work as intended

# Design Checklist
---
1. What is the abstract thing you are trying to represent? Often the answer will be in 
terms of sets, sequences, and finite maps.   
`Answer:`
- Creating a polymorphic 2D array and it’s supposed to be called Array2 and 
suppose to allow users to use array elements of any type. This Array2 is supposed 
to support images.  
 
2. What functions will you offer, and what are the contracts of that those functions 
must meet?  
    `Answer:` 
        
        fn new(width: usize, height: usize) -> Array2<T> { todo!()} 
 
            -> creates a new Array2 with specific width and height 
 
        fn from_row_major(vec: Vec<T>, width: usize, height: usize) -> Array2<T> { todo!() }    
 
            -> constructs an Array2 from a dimensional Vec<T> and mapping the elements in row-major order. 
 
        fn from_col_major(vec: Vec<T>, width: usize, height: usize) -> Array2<T> { todo!()} 
 
            -> constructs an Array2 from a dimensional Vec<T> and mapping the elements in column-major order. 
 
        fn get_data(x_cord: usize, y_cord: usize) -> Option<&T> {  todo!()  }  
 
            -> access specific cordinates and returns the value at the at specific index 
 
        fn iter_col_major(&self) -> Iter<T> { todo!() } -> returns an iterator that allows for iterations over an array in column-major order 
 
        fn iter_row_major(&self) -> Iter<T> { todo!() } 
 
            -> returns an iterator that allows for iterations over an array in row-major order 
         
 
4. What representation will you use, and what invariants will it satisfy? (This question 
is especially important to answer precisely.)  
     `Answer:`
                
         The Array2 will be represented as a struct containing a one-dimensional Vec<T> to store the elements. 
         Invariants: 
            • The height and width of the array will always be greater than 0. 
            • There needs to be enough vectors in the 2D array to be transferred into the 1D vector.  
            • row * width + column -> row-major order. 
            • The iterator should visit every element in every row until the last row. 
            • column * height + row -> column-major order. The iterator should visit every element in every column until the last column.  
 
# Hours worked on Assignment
---
- I worked approximately 2-3 hours on the design document and i'm estimating a total of 25 hours on the full program.

