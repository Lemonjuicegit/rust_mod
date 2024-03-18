use std::fs::File;
use std::io::{Read,Result,BufReader};
use encoding::{DecoderTrap, Encoding,all::GBK};

#[allow(dead_code)]
pub fn read_gbk(filename: &str) -> Result<String> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new(); 
    reader.read_to_end(&mut buffer)?;
    let str_builder = GBK.decode(&buffer, DecoderTrap::Strict).unwrap();
    Ok(str_builder)
}

