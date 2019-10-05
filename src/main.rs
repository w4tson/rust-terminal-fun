#![feature(rustc_private)]

#[macro_use]
extern crate serde_derive;
extern crate hyperlocal;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;
extern crate reqwest;
extern crate rand;
extern crate chrono;


#[allow(unused_imports)]
use crate::docker::hello_docker;

pub mod docker;
pub mod github;
pub mod ui; 
pub mod devoxx; 

fn main() -> Result<(), failure::Error> {
    ui::repos_screen::run()
}
