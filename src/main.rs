extern crate hyperlocal;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;
extern crate reqwest;
extern crate chrono;


#[allow(unused_imports)]
use crate::docker::hello_docker;

use std::io;
use std::io::Write;

use structopt::StructOpt;


pub mod docker;
pub mod github;
pub mod ui; 
pub mod devoxx; 


fn main() -> Result<(), failure::Error> {
    let options = Options::from_args();
    let result = ui::screen::run(options);
    
    if let Err(e) = result {
        let mut stdout = io::stdout();
        stdout.write_all(format!("{:#?}", e).as_bytes())?;
    } 
   
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "devoxx-schedule", about = "A command line tool to browse the Devoxx schedule")]
pub struct Options {
    #[structopt(short, long)]
    /// Uses the schedule from local disk, instead of the Devoxx API
    pub offline: bool

}
