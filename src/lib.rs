mod defs;
mod formats;
mod raw_buffer;
mod read_interface;

pub use defs::ImageInfoError;
pub use defs::ImageInfoResult;
pub use defs::ImageSize;
use formats::try_avif_heic;
use formats::try_bmp;
use formats::try_cur_ico;
use formats::try_dds;
use formats::try_gif;
use formats::try_hdr;
use formats::try_icns;
use formats::try_jp2_jpx;
use formats::try_jpg;
use formats::try_ktx;
use formats::try_png;
use formats::try_psd;
use formats::try_qoi;
use formats::try_tga;
use formats::try_tiff;
use formats::try_webp;
use raw_buffer::RawBuffer;
use read_interface::ReadInterface;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek, SeekFrom};
use std::path::Path;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum ImageFormat {
    AVIF,
    HEIC,
    BMP,
    CUR,
    ICO,
    DDS,
    GIF,
    HDR,
    ICNS,
    JP2,
    JPX,
    JPEG,
    KTX,
    PNG,
    PSD,
    QOI,
    TIFF,
    WEBP,
    TGA,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ImageInfo {
    pub format: ImageFormat,
    pub ext: &'static str,
    pub full_ext: &'static str,
    pub mimetype: &'static str,
    pub size: ImageSize,
    pub entry_sizes: Vec<ImageSize>,
}

impl ImageInfo {
    pub fn from_reader<R>(reader: &mut R) -> ImageInfoResult<ImageInfo>
    where
        R: BufRead + Seek,
    {
        let length = reader.seek(SeekFrom::End(0))? as usize;
        let mut ri = ReadInterface::from_reader(reader, length);

        if let Ok(image_info) = try_avif_heic(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_bmp(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_cur_ico(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_dds(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_gif(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_hdr(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_icns(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_jp2_jpx(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_jpg(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_ktx(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_png(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_psd(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_qoi(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_tiff(&mut ri, length) {
            return Ok(image_info);
        }

        if let Ok(image_info) = try_webp(&mut ri, length) {
            return Ok(image_info);
        }

        // !!! keep tga last !!!
        if let Ok(image_info) = try_tga(&mut ri, length) {
            return Ok(image_info);
        }

        Err(ImageInfoError::UnrecognizedFormat)
    }

    pub fn from_file(file: &File) -> ImageInfoResult<ImageInfo> {
        let mut reader = BufReader::new(file);
        Self::from_reader(&mut reader)
    }

    pub fn from_file_path(filepath: impl AsRef<Path>) -> ImageInfoResult<ImageInfo> {
        let file = File::open(filepath)?;
        Self::from_file(&file)
    }

    pub fn from_raw_data(data: &[u8]) -> ImageInfoResult<ImageInfo> {
        let mut reader = BufReader::new(Cursor::new(data));
        Self::from_reader(&mut reader)
    }
}
