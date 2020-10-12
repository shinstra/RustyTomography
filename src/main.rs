

use ndarray::Array2;
use image::GrayImage;

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
    print!("min {}, max {}\n",vmin, vmax);
    let mut raw: Vec<u8> = Vec::with_capacity(raw_f64.len());
    for i in 0..raw_f64.len(){
        raw.push( ((raw_f64[i] - vmin) / (vmax - vmin) * 255.0) as u8 );  // Maximize dynamic range.
    }
    print!("height {}, width {}, len {}\n", height, width, raw.len());

    let img = GrayImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions");

    let result = img.save(path);
}


/// Generates 2D Shepp-Logan Phantom image of specified size.
/// 
/// https://en.wikipedia.org/wiki/Shepp%E2%80%93Logan_phantom
/// 
pub fn shepp_logan_2d(size: usize) -> Array2<f64>{

    if size < 1 {
        panic!("Cannot make phantom with zero size!")
    }

    // Initialise Array
    let mut frame = Array2::<f64>::zeros((size, size));  // row-major

    fn inside_ellipse(point: (f64, f64), center: (f64, f64), major: f64, minor: f64, theta: f64) -> bool {
        // https://stackoverflow.com/questions/7946187/point-and-ellipse-rotated-position-test-algorithm
        let x = point.0 - center.0;
        let y = point.1 - center.1;
        let a = major;
        let b = minor;
        let cos0 = theta.cos();
        let sin0 = theta.sin();
        let val = ((cos0 * x + sin0 * y) / a).powi(2) + ((sin0 * x - cos0 * y) / b).powi(2);
        return val <= 1.0
    }

    let dp: f64 = 2.0 / (size as f64);
    let p0: f64 = - 1.0 + dp / 2.0; 


    for h in 0..size {
        for w in 0..size {

            let p: (f64, f64) = (p0 + (h as f64) * dp, p0 + (w as f64) * dp);  // voxel center

            // Ellipse (a)
            if inside_ellipse(p, (0.0, 0.0), 0.69, 0.92, 0.0){
                frame[(h, w)] += 2.0;
            }
            // Ellipse (b)
            if inside_ellipse(p, (0.0, -0.0184), 0.6624, 0.874, 0.0){
                frame[(h, w)] += -0.98;
            }
            // Ellipse (c)
            if inside_ellipse(p, (0.22, 0.0), 0.11, 0.31, -0.3141592653589793){  // -18 deg
                frame[(h, w)] += -0.02;
            }
            // Ellipse (d)
            if inside_ellipse(p, (-0.22, 0.0), 0.16, 0.41, 0.3141592653589793){  // +18deg
                frame[(h, w)] += -0.02;
            }
            // Ellipse (e)
            if inside_ellipse(p, (0.0, 0.35), 0.21, 0.25, 0.0){
                frame[(h, w)] += 0.01;
            }
            // Ellipse (f)
            if inside_ellipse(p, (0.0, 0.1), 0.046, 0.046, 0.0){
                frame[(h, w)] += 0.01;
            }
            // Ellipse (g)
            if inside_ellipse(p, (0.0, -0.1), 0.046, 0.046, 0.0){
                frame[(h, w)] += 0.01;
            }
            // Ellipse (h)
            if inside_ellipse(p, (-0.08, -0.605), 0.046, 0.023, 0.0){
                frame[(h, w)] += 0.01;
            }
            // Ellipse (i)
            if inside_ellipse(p, (0.0, -0.605), 0.023, 0.023, 0.0){
                frame[(h, w)] += 0.01;
            }
            // Ellipse (j)
            if inside_ellipse(p, (0.06, -0.605), 0.023, 0.046, 0.0){
                frame[(h, w)] += 0.01;
            }
        }
    }
    print!("point {}\n", frame[(200, 200)]);

    return frame
}