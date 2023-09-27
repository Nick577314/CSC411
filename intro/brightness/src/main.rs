

use csc411_image::{Read,GrayImage};
use std::env;
fn main(){

    let input = env::args().nth(1);
    //assert!(env::args().len() == 2);
    let _img = GrayImage::read(input.as_deref()).unwrap();
    let _dnm = _img.denominator as u64;
    let brightness: f64;
    let mut temp_bright = 0 as u64;
    let total_pixel = (_img.height * _img.width) as u64;
    for _pixel in _img.pixels {
       
        // loop through the pixels add them up and divide by denominator
       temp_bright += (_pixel.value) as u64;
      
    }
    brightness = (temp_bright as f64) / (_dnm as f64) / (total_pixel as f64);
    
    println!("{:.3}",brightness);

}
