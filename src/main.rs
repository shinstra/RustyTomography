

mod phantom;


use ndarray::Array2;
use image::GrayImage;

use phantom::shepp_logan_2d;



fn main() {
    let arr = shepp_logan_2d(500);
    export_image(String::from("C:\\Users\\batemanc\\Documents\\source\\test.png"), arr);
    println!("Hello, world!");
}


pub fn export_image(path: String, arr: Array2<f64>) {

    // Convert array for saving
    let (height, width) = arr.dim();
    let raw_f64 = arr.into_raw_vec();
    let vmin = raw_f64.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).expect("Invalid Data in Array."); 
    let vmax = raw_f64.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).expect("Invalid Data in Array."); 
    let mut raw: Vec<u8> = Vec::with_capacity(raw_f64.len());
    for i in 0..raw_f64.len(){
        raw.push( ((raw_f64[i] - vmin) / (vmax - vmin) * 255.0) as u8 );  // Maximize dynamic range.
    }

    // Convert to image object to save.
    let img = GrayImage::from_raw(width as u32, height as u32, raw)
        .expect("Image buffer does not match image dimensions.");
    img.save(path).expect("Error saving image");
}


