extern crate fitsio;
extern crate ndarray;
extern crate num_traits;

use fitsio::FitsFile;
use fitsio::fitsfile::ImageDescription;
use fitsio::types::ImageType;
use fitsio::types::HduInfo;
use fitsio::Error;
use fitsio::Result;
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
        ImageType::SBYTE_IMG
    }
}

impl TypeToImageType for u8 {
    fn get_img_type() -> ImageType {
        ImageType::BYTE_IMG
    }
}

impl TypeToImageType for i16 {
    fn get_img_type() -> ImageType {
        ImageType::SHORT_IMG
    }
}

impl TypeToImageType for u16 {
    fn get_img_type() -> ImageType {
        ImageType::USHORT_IMG
    }
}

impl TypeToImageType for i32 {
    fn get_img_type() -> ImageType {
        ImageType::LONG_IMG
    }
}

impl TypeToImageType for u32 {
    fn get_img_type() -> ImageType {
        ImageType::ULONG_IMG
    }
}

impl TypeToImageType for i64 {
    fn get_img_type() -> ImageType {
        ImageType::LONGLONG_IMG
    }
}

impl TypeToImageType for f32 {
    fn get_img_type() -> ImageType {
        ImageType::FLOAT_IMG
    }
}

impl TypeToImageType for f64 {
    fn get_img_type() -> ImageType {
        ImageType::DOUBLE_IMG
    }
}

pub fn read_img<T>(fname: String, n: usize) -> Result<ArrayD<T>>
where
    T: Float + NumCast + ReadWriteImage,
{
    let mut fits_file = fitsio::FitsFile::open(fname)?;
    let hdu = fits_file.hdu(n)?;
    let shape = match hdu.info {
        HduInfo::ImageInfo { ref shape, .. } => shape.clone(),
        _ => return Err(Error::Message("Not image".to_string())),
    };

    let data = hdu.read_image(&mut fits_file)?;

    match ArrayD::<T>::from_shape_vec(shape.into_dimension(), data) {
        Ok(x) => Ok(x),
        Err(_) => Err(Error::Message("err".to_string())),
    }
}

pub fn write_img<T>(fname: String, data: &ArrayD<T>) -> Result<()>
where
    T: Float + NumCast + ReadWriteImage + TypeToImageType,
{
    let mut fits_file = fitsio::FitsFile::create(fname)?;
    let shape = data.shape();
    let img_desc = ImageDescription {
        data_type: <T as TypeToImageType>::get_img_type(),
        dimensions: shape,
    };

    let hdu = fits_file.create_image("".to_string(), &img_desc)?;
    let mut data1 = Vec::<T>::new();
    for x in data.into_iter() {
        data1.push(*x);
    }

    hdu.write_section(&mut fits_file, 0, data1.len(), &data1);
    Ok(())
}
