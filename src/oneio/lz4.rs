use crate::oneio::OneIOCompression;
use crate::OneIoError;
use lz4::Decoder;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub(crate) struct OneIOLz4;

impl OneIOCompression for OneIOLz4 {
    fn get_reader(raw_reader: Box<dyn Read>) -> Result<Box<dyn BufRead>, OneIoError> {
        let reader = Box::new(Decoder::new(raw_reader).unwrap());
        Ok(Box::new(BufReader::new(reader)))
    }

    fn get_writer(_raw_writer: BufWriter<File>) -> Result<Box<dyn Write>, OneIoError> {
        Err(OneIoError::Unsupported("lz4 writer is not currently supported.".to_string()))
    }
}
