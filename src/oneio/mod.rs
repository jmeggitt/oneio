#[cfg(feature = "bz")]
mod bzip2;
#[cfg(feature = "gz")]
mod gzip;
#[cfg(feature = "lz")]
mod lz4;

use crate::OneIoError;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub trait OneIOCompression {
    fn get_reader(raw_reader: Box<dyn Read>) -> Result<Box<dyn BufRead>, OneIoError>;
    fn get_writer(raw_writer: BufWriter<File>) -> Result<Box<dyn Write>, OneIoError>;
}

fn get_reader_raw(path: &str) -> Result<Box<dyn BufRead>, OneIoError> {
    #[cfg(feature = "remote")]
    let raw_reader: Box<dyn Read> = match path.starts_with("http") {
        true => {
            let response = reqwest::blocking::get(path)?;
            Box::new(response)
        }
        false => Box::new(std::fs::File::open(path)?),
    };
    #[cfg(not(feature = "remote"))]
    let raw_reader: Box<dyn Read> = Box::new(std::fs::File::open(path)?);
    let reader = Box::new(raw_reader);
    Ok(Box::new(BufReader::new(reader)))
}

pub fn get_reader(path: &str) -> Result<Box<dyn BufRead>, OneIoError> {
    #[cfg(feature = "remote")]
    let raw_reader: Box<dyn Read> = match path.starts_with("http") {
        true => {
            let response = reqwest::blocking::get(path)?;
            Box::new(response)
        }
        false => Box::new(std::fs::File::open(path)?),
    };
    #[cfg(not(feature = "remote"))]
    let raw_reader: Box<dyn Read> = Box::new(std::fs::File::open(path)?);

    let file_type = path
        .split(".")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .clone();
    match file_type {
        #[cfg(feature = "gz")]
        "gz" | "gzip" => gzip::OneIOGzip::get_reader(raw_reader),
        #[cfg(feature = "bz")]
        "bz2" | "bz" => bzip2::OneIOBzip2::get_reader(raw_reader),
        #[cfg(feature = "lz4")]
        "lz4" | "lz" => lz4::OneIOLz4::get_reader(raw_reader),
        _ => {
            // unknown file type of file {}. try to read as uncompressed file
            let reader = Box::new(raw_reader);
            Ok(Box::new(BufReader::new(reader)))
        }
    }
}

/// get file reader with local cache.
///
/// parameters:
/// * `path`: file path to open, remote or local
/// * `cache_dir`: path str to cache directory
/// * `cache_file_name`: optional file name for cache file, default to use the same filename as the to-read file
/// * `force_cache`: whether to force refresh cache file if a local cache file already exists
pub fn get_cache_reader(
    path: &str,
    cache_dir: &str,
    cache_file_name: Option<String>,
    force_cache: bool,
) -> Result<Box<dyn BufRead>, OneIoError> {
    let dir_path = std::path::Path::new(cache_dir);

    // There is no harm in calling this if the directory already exists
    if let Err(e) = std::fs::create_dir_all(dir_path) {
        return Err(OneIoError::Cache(format!("cache directory creation failed: {}", e)))
    }

    let cache_file_path = match cache_file_name {
        None => {
            let file_name = path
                .split('/')
                .collect::<Vec<&str>>()
                .into_iter()
                .last()
                .unwrap()
                .to_string();
            format!("{}/{}", cache_dir, file_name)
        }
        Some(p) => p,
    };

    // if cache file already exists
    if !force_cache && std::path::Path::new(cache_file_path.as_str()).exists() {
        return get_reader(cache_file_path.as_str());
    }

    // read all to cache file, no encode/decode happens
    let mut reader = get_reader_raw(path)?;
    let mut data: Vec<u8> = vec![];
    reader.read_to_end(&mut data)?;
    let mut writer = get_writer_raw(cache_file_path.as_str())?;
    writer.write_all(&data).unwrap();
    drop(writer);

    // return reader from cache file
    get_reader(cache_file_path.as_str())
}

fn get_writer_raw(path: &str) -> Result<Box<dyn Write>, OneIoError> {
    let output_file = BufWriter::new(File::create(path)?);
    Ok(Box::new(BufWriter::new(output_file)))
}

pub fn get_writer(path: &str) -> Result<Box<dyn Write>, OneIoError> {
    let output_file = BufWriter::new(File::create(path)?);

    let file_type = path
        .split(".")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .clone();
    match file_type {
        #[cfg(feature = "gz")]
        "gz" | "gzip" => gzip::OneIOGzip::get_writer(output_file),
        #[cfg(feature = "bz")]
        "bz2" | "bz" => bzip2::OneIOBzip2::get_writer(output_file),
        #[cfg(feature = "lz4")]
        "lz4" | "lz" => lz4::OneIOLz4::get_writer(output_file),
        _ => Ok(Box::new(BufWriter::new(output_file))),
    }
}
