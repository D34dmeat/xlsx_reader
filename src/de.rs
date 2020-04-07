extern crate quick_xml;
use quick_xml::{events::Event,Reader};



pub fn local_deserialize<T>(lisp:T)->Result<T,&'static str>{
Ok(lisp)
}


