#![allow(unused)]
use array2::Array2;
use clap::Parser;
use csc411_image::{Read, Rgb, RgbImage, Write};
use std::{env};
use std::time::{Instant, Duration};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Rotation
    #[clap(short = 'r', long = "rotate")]
    rotate: Option<u32>,
    // Transposition mode (row-major or col-major)
    #[clap(long = "row-major")]
    row_major: bool,

    #[clap(long = "col-major")]
    col_major: bool,
    // Flip
    #[clap(long = "flip", required = false)]
    flip: Option<String>,
    // Transposition
    #[clap(long = "transpose")]
    transpose: bool,
    
    file: Option<String>,

    #[clap(long = "benchmark",required = false)]
    benchmark: bool
}

fn main() {
    let args = Args::parse();
    let rotate = args.rotate;
    let row_major = args.row_major;
    let col_major = args.col_major;
    let transpose = args.transpose;
    let flip = args.flip;
    let benchmark = args.benchmark;
    let img = RgbImage::read(args.file.as_deref()).unwrap();

    let data: Vec<Rgb> = img.pixels.clone(); 
    let width: usize = img.width as usize;
    let height: usize = img.height as usize;
    let denom: u16 = img.denominator;

    let temp_image = Array2::from_row_major(data, width, height);

    if row_major && col_major {
        eprintln!("ERROR: cannot use both col-major and row-major flags at the same time");
        std::process::exit(1);
    }
    
    let now = Instant::now();
    
    if row_major {
        let iter_order = 1;
      
        image_rotations(
            &temp_image,
            width as u32,
            height as u32,
            denom,
            rotate,
            iter_order,
        );
        bench_mark(benchmark, now);

        if transpose {
            other_image_processes(&temp_image,width as u32 , height as u32,iter_order, denom, flip.clone());
        }
        
        if let Some(operation) = flip.clone() {
            other_image_processes(&temp_image,width as u32 , height as u32,iter_order, denom,flip.clone());
        }
    }
    if col_major {
        
        let iter_order = 2;
      
        image_rotations(
            &temp_image,
            width as u32,
            height as u32,
            denom,
            rotate,
            iter_order,
        );
        bench_mark(benchmark, now);
        if transpose {
            other_image_processes(&temp_image,width as u32 , height as u32,iter_order, denom, flip.clone());
        }
        if let Some(operation) = flip.clone() {
            other_image_processes(&temp_image,width as u32 , height as u32,iter_order, denom, flip.clone());
        }
    }
    
    
}

fn bench_mark(benchmark: bool ,now: Instant ){

    if benchmark{
        let elapsed = now.elapsed();
        eprintln!("{:.2?}", elapsed);
        }
}
fn image_rotations(
        temp_image: &Array2<Rgb>,
        width: u32,
        height: u32,
        denom: u16,
        rotate: Option<u32>,
        iter_order: i32,
    ) {
        if let Some(angle) = rotate { 
            // if some value angle is equal to the value stored in rotate 
            // match the angle value and call the corresponding rotate function accordingly
            let (rotated_array, rotated_width, rotated_height) = match angle {
                90 => (rotate90(&temp_image, iter_order), height, width),
                180 => (rotate180(&temp_image, iter_order), width, height),
                270 => (rotate270(&temp_image, iter_order), height, width),
                0 => (rotate0(&temp_image, iter_order),width,height),

                _ => { //handles incorrect angle input
                    
                    return;
                }
            };
    
            let pixels = rotated_array.vec;
            let rotated_image = create_rgbimage(pixels, rotated_width, rotated_height, denom);
        }
    }
fn rotate0<T: Clone> (arr: &Array2<T>, iter_order: i32) -> Array2<T> {

    let width = arr.get_width();
    let height = arr.get_height();
    let mut rotated_image = Array2::blank_slate(width, height, arr.vec[0].clone());

    if iter_order == 1 {
        for (col, row, val) in arr.iter_row_major() {
            let rotated_col = col;
            let rotated_row = row;

            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    } else if iter_order == 2 {
        for (col, row, val) in arr.iter_col_major() {
            let rotated_col = col;
            let rotated_row = row;

            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    }
    rotated_image
}

fn rotate90<T: Clone>(arr: &Array2<T>, iter_order: i32) -> Array2<T> {
  
    let width = arr.get_width();
    let height = arr.get_height();
    let mut rotated_image = Array2::blank_slate(height, width, arr.vec[0].clone());

    if iter_order == 1 {
        for (col, row, val) in arr.iter_row_major() {
            let rotated_col = height - row - 1;
            let rotated_row = col;

            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    } else if iter_order == 2 {
        for (col, row, val) in arr.iter_col_major() {
            let rotated_col = height - row - 1;
            let rotated_row = col;

            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    }
  
    rotated_image
}

fn rotate180<T: Clone>(arr: &Array2<T>, iter_order: i32) -> Array2<T> {
    let width = arr.get_width();
    let height = arr.get_height();
    let mut rotated_image = Array2::blank_slate(width, height, arr.vec[0].clone());

    if iter_order == 1 {
        for (col, row, val) in arr.iter_row_major() {
            let rotated_col = width - col - 1;
            let rotated_row = height - row - 1;
            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    } else if iter_order == 2 {
        for (col, row, val) in arr.iter_col_major() {
            let rotated_col = width - col - 1;
            let rotated_row = height - row - 1;
            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    }

    rotated_image
}

fn rotate270<T: Clone>(arr: &Array2<T>, iter_order: i32) -> Array2<T> {
    let width = arr.get_width();
    let height = arr.get_height();
    let mut rotated_image = Array2::blank_slate(height, width, arr.vec[0].clone());

    if iter_order == 1 {
        for (col, row, val) in arr.iter_row_major() {
            let rotated_col = row;
            let rotated_row = width - col - 1;
            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    } else if iter_order == 2 {
        for (col, row, val) in arr.iter_col_major() {
            let rotated_col = row;
            let rotated_row = width - col - 1;
            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    }

    rotated_image
}

fn other_image_processes(temp_image: &Array2<Rgb>, width:u32, height: u32,iter_order: i32, denom: u16,flip: Option<String>){

    if let Some(flip_method) = flip{
        let (rotated_array, rotated_width, rotated_height) = match flip_method.as_str() {
            "horizontal" => (flip_horizontal(&temp_image, iter_order), height, width),
            "vertical" => (flip_vertical(&temp_image, iter_order), height, width),
           
            _ => { //handles incorrect syntax
                
                return;
            }
        };
        let pixels = rotated_array.vec;
        let rotated_image = create_rgbimage(
            pixels,
            rotated_height,
            rotated_width,
            denom,
        );
    }else{

        let rotated_array = transpose(&temp_image, iter_order);
            let pixels = rotated_array.vec;
            let rotated_image = create_rgbimage(
                pixels,
                height,
                width,
                denom,
            );

    }

    }

fn transpose<T: Clone>(arr: &Array2<T>, iter_order: i32) -> Array2<T> {
    let width = arr.get_width();
    let height = arr.get_height();
    let mut rotated_image = Array2::blank_slate(height, width, arr.vec[0].clone());
    if iter_order == 1 {
        for (col, row, val) in arr.iter_row_major() {
            let rotated_col = row; 
            let rotated_row = col;
            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    } else if iter_order == 2 {
        for (col, row, val) in arr.iter_col_major() {
            let rotated_col = row;
            let rotated_row = col;
            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    }
    
    rotated_image
}

fn create_rgbimage(new_pixels: Vec<Rgb>, width: u32, height: u32, denom: u16) {
    // create new RgbImage
    let temp_image = RgbImage {
        pixels: new_pixels,
        width: width,
        height: height,
        denominator: denom,
    };
    // write to standard in
    temp_image.write(None);
}
 
fn flip_horizontal<T: Clone>(arr: &Array2<T>, iter_order: i32) -> Array2<T>{

    // left to right (mirror image) horizontally
    let width = arr.get_width();
    let height = arr.get_height();
    let mut rotated_image = Array2::blank_slate(width, height, arr.vec[0].clone());
    if iter_order == 1 {
        for (col, row, val) in arr.iter_row_major() {
            let rotated_col = width - col -1;
            let rotated_row = row;
            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    } else if iter_order == 2 {
        for (col, row, val) in arr.iter_col_major() {
            let rotated_col = width - col - 1;
            let rotated_row = row;
            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    }
    
    rotated_image
    

}

 fn flip_vertical<T: Clone>(arr: &Array2<T>, iter_order: i32) -> Array2<T>{

    // left to right (mirror image) horizontally
    let width = arr.get_width();
    let height = arr.get_height();
    let mut rotated_image = Array2::blank_slate(width, height, arr.vec[0].clone());
    if iter_order == 1 {
        for (col, row, val) in arr.iter_row_major() {
            let rotated_col = col;
            let rotated_row = height - row - 1;
            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    } else if iter_order == 2 {
        for (col, row, val) in arr.iter_col_major() {
            let rotated_col = col;
            let rotated_row = height - row - 1;
            if let Some(value) = rotated_image.get_mut(rotated_col, rotated_row) {
                *value = val.clone();
            }
        }
    }
    
    rotated_image
 }
