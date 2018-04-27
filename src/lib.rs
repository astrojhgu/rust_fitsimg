extern crate fitsio;
extern crate ndarray;
extern crate num_traits;
use std::iter::FromIterator;
use std::fs::remove_file;
use fitsio::FitsFile;
use fitsio::fitsfile::ImageDescription;
use fitsio::types::ImageType;
use fitsio::types::HduInfo;
use fitsio::errors::Error;
use fitsio::errors::Result;
use fitsio::fitsfile::ReadWriteImage;

use num_traits::Float;
use num_traits::NumCast;

use ndarray::IntoDimension;
use ndarray::ArrayD;

pub trait TypeToImageType {
    fn get_img_type() -> ImageType;
}

impl TypeToImageType for i8 {
    fn get_img_type() -> ImageType {
        ImageType::Byte
    }
}

impl TypeToImageType for u8 {
    fn get_img_type() -> ImageType {
        ImageType::UnsignedByte
    }
}

impl TypeToImageType for i16 {
    fn get_img_type() -> ImageType {
        ImageType::Short
    }
}

impl TypeToImageType for u16 {
    fn get_img_type() -> ImageType {
        ImageType::UnsignedShort
    }
}

impl TypeToImageType for i32 {
    fn get_img_type() -> ImageType {
        ImageType::Long
    }
}

impl TypeToImageType for u32 {
    fn get_img_type() -> ImageType {
        ImageType::UnsignedLong
    }
}

impl TypeToImageType for i64 {
    fn get_img_type() -> ImageType {
        ImageType::LongLong
    }
}

impl TypeToImageType for f32 {
    fn get_img_type() -> ImageType {
        ImageType::Float
    }
}

impl TypeToImageType for f64 {
    fn get_img_type() -> ImageType {
        ImageType::Double
    }
}

pub fn read_img<T>(fname: String, n: usize) -> Result<ArrayD<T>>
where
    T: Float + NumCast + ReadWriteImage,
{
    let mut fits_file = fitsio::FitsFile::open(fname)?;
    let hdu = fits_file.hdu(n)?;
    let mut shape = match hdu.info {
        HduInfo::ImageInfo { ref shape, .. } => {
            println!("{:?}", shape);
            shape.clone()
        }
        _ => return Err(Error::Message("Not image".to_string())),
    };

    //shape.reverse();
    let data = hdu.read_image(&mut fits_file)?;

    match ArrayD::<T>::from_shape_vec(shape.into_dimension(), data) {
        Ok(x) => {
            println!("{:?}", x.shape());
            Ok(x)
        }
        Err(_) => Err(Error::Message("err".to_string())),
    }
}

pub fn write_img<T>(fname: String, data: &ArrayD<T>) -> Result<()>
where
    T: Float + NumCast + ReadWriteImage + TypeToImageType,
{
    let mut shape = data.shape().to_vec();
    //shape.reverse();
    let img_desc = ImageDescription {
        data_type: <T as TypeToImageType>::get_img_type(),
        dimensions: shape.as_slice(),
    };

    let mut fits_file = {
        remove_file(&fname);
        match fitsio::FitsFile::create(fname)
            .with_custom_primary(&img_desc)
            .open()
        {
            Ok(x) => x,
            Err(x) => {
                println!("{}", x);
                return Err(x);
            }
        }
    };
    //let hdu = fits_file.create_image("".to_string(), &img_desc)?;
    let hdu = fits_file.current_hdu()?;
    let mut data1 = Vec::<T>::new();
    for x in data.into_iter() {
        data1.push(*x);
    }

    hdu.write_section(&mut fits_file, 0, data1.len(), &data1);
    Ok(())
}
