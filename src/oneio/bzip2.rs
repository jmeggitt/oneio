use crate::oneio::OneIOCompression;
use crate::OneIoError;
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub(crate) struct OneIOBzip2;

impl OneIOCompression for OneIOBzip2 {
    fn get_reader(raw_reader: Box<dyn Read>) -> Result<Box<dyn BufRead>, OneIoError> {
        let reader = Box::new(BzDecoder::new(raw_reader));
        Ok(Box::new(BufReader::new(reader)))
    }

    fn get_writer(raw_writer: BufWriter<File>) -> Result<Box<dyn Write>, OneIoError> {
        Ok(Box::new(BzEncoder::new(raw_writer, Compression::default())))
    }
}
