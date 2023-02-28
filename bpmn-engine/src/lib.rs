#![allow(unused_imports, dead_code, unused_variables)]

use definitions::Definitions;
use std::fs::read_to_string;
use strong_xml::XmlRead;

pub mod definitions;
pub mod engine;
mod graph;

pub fn parse_xml(input: &str) -> Definitions {
    Definitions::from_str(&input).unwrap()
}
