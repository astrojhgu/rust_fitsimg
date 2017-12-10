extern crate fitsimg;
extern crate ndarray;




fn main(){
    let aa=ndarray::arr2(&[[1.0,2.0],[2.0,3.0]]).into_dyn();
    fitsimg::write_img("a.fits".to_string(), &aa);
}