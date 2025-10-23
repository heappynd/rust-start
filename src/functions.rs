use crate::models::structs::HousePrice;
// use csv::Writer;
// use csv::ReaderBuilder;
// use csv::{Writer, ReaderBuilder};
use csv::*;

pub fn read_csv(path: String) -> Vec<HousePrice> {
    let mut rdr = Reader::from_path(path);
    vec![]
}
