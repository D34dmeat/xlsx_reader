extern crate zip;
//extern crate serde_xml_rs;
extern crate quick_xml;
#[macro_use] extern crate serde_derive;

pub mod reader;

pub use reader::parse_xlsx;

#[cfg(test)]
mod tests;
