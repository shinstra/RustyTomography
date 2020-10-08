use ndarray::Array2;

fn main() {
    shepp_logan_2d(500);
    println!("Hello, world!");
}


/// Generates 2D Shepp-Logan Phantom image of specified size.
/// 
/// https://en.wikipedia.org/wiki/Shepp%E2%80%93Logan_phantom
/// 
pub fn shepp_logan_2d(size: usize){

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

    let p0: f64 = - (size as f64) / 2.0; 
    let dp: f64 = 2.0 / (size as f64);

    for h in 0..size {
        for w in 0..size {

            let p: (f64, f64) = (p0 + (h as f64) * dp, p0 + (w as f64) * dp);

            // Ellipse (a)
            if inside_ellipse(p, (0.0, 0.0), 0.69, 0.92, 0.0){
                frame[(h, w)] += 2.0
            }
            // Ellipse (b)
            if inside_ellipse(p, (0.0, -0.0184), 0.6624, 0.874, 0.0){
                frame[(h, w)] += -0.98
            }
            // Ellipse (c)
            if inside_ellipse(p, (0.22, 0.0), 0.11, 0.31, -18.0){
                frame[(h, w)] += -0.02
            }
            // Ellipse (d)
            if inside_ellipse(p, (-0.22, 0.0), 0.16, 0.41, 18.0){
                frame[(h, w)] += -0.02
            }
            // Ellipse (e)
            if inside_ellipse(p, (0.0, 0.35), 0.21, 0.25, 0.0){
                frame[(h, w)] += 0.01
            }
            // Ellipse (f)
            if inside_ellipse(p, (0.0, 0.1), 0.046, 0.046, 0.0){
                frame[(h, w)] += 0.01
            }
            // Ellipse (g)
            if inside_ellipse(p, (0.0, -0.1), 0.046, 0.046, 0.0){
                frame[(h, w)] += 0.01
            }
            // Ellipse (h)
            if inside_ellipse(p, (-0.08, -0.605), 0.046, 0.023, 0.0){
                frame[(h, w)] += 0.01
            }
            // Ellipse (i)
            if inside_ellipse(p, (0.0, -0.605), 0.023, 0.023, 0.0){
                frame[(h, w)] += 0.01
            }
            // Ellipse (j)
            if inside_ellipse(p, (0.06, -0.605), 0.023, 0.046, 0.0){
                frame[(h, w)] += 0.01
            }

        }

    }








}