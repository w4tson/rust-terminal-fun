#[macro_use]
extern crate serde_derive;
extern crate hyperlocal;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;
extern crate reqwest;
extern crate chrono;


#[allow(unused_imports)]
use crate::docker::hello_docker;
use std::io;
use termion::raw::IntoRawMode;
use std::io::Write;

pub mod docker;
pub mod github;
pub mod ui; 
pub mod devoxx; 

fn main() -> Result<(), failure::Error> {
    let result = ui::repos_screen::run();
    
    if let Err(e) = result {
        let mut stdout = io::stdout();
        stdout.write_all(format!("{:#?}", e).as_bytes())?;
    } 
   
    Ok(())
}
