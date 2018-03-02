extern crate fitsimg;
extern crate ndarray;

use std::convert::From;

fn main() {
    //let aa = ndarray::arr2(&[[1.0, 2.0], [2.0, 3.0]]).into_dyn();
    let mut aa = ndarray::Array3::<f64>::zeros((8, 16, 32)).into_dyn();
    println!("{:?}", aa.shape());
    println!("{} {}", aa.shape()[0], aa.shape()[1]);
    let mut cnt = 0.0;
    for i in 0..aa.shape()[0] {
        for j in 0..aa.shape()[1] {
            for k in 0..aa.shape()[2] {
                aa[[i, j, k]] = (i + j + k) as f64;
                cnt = cnt + 1.0;
            }
        }
    }

    fitsimg::write_img("a.fits".to_string(), &aa);
    let bb = fitsimg::read_img("a.fits".to_string(), 0).unwrap();
    assert_eq!(bb, aa);
    println!("{:?}", bb.shape());
}
