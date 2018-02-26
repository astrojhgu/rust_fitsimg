extern crate fitsimg;
extern crate ndarray;

use std::convert::From;

fn main() {
    //let aa = ndarray::arr2(&[[1.0, 2.0], [2.0, 3.0]]).into_dyn();
    let mut aa = ndarray::Array2::<f64>::zeros((1024, 1024)).into_dyn();

    for i in 0..aa.shape()[0] {
        for j in 0..aa.shape()[1] {
            aa[[i, j]] = (i + j) as f64;
        }
    }

    fitsimg::write_img("a.fits".to_string(), &aa);
    let bb = fitsimg::read_img("a.fits".to_string(), 0).unwrap();
    assert_eq!(bb, aa);
}
