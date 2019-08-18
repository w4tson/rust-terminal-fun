use hyperlocal::{ Uri, UnixConnector};
use hyper::{Client, rt};
use futures::Stream;
use futures::Future;
use std::io::{self, Write};


pub fn hello_docker() {
    eprintln!("Hello Docker ");

    let client = Client::builder()
        .keep_alive(false)
        .build::<_, ::hyper::Body>(UnixConnector::new());
    
    let url = Uri::new("/var/run/docker.sock", "/info").into();

    let work = client
        .get(url)
        .and_then(|res|{
            eprintln!("response = {:#?}", res);
            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        }).map(|_| {
            println!("hello")
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        });
    
    rt::run(work);
}



