use std::env::var;
use std::io;

use ureq::Error;


pub fn get_input(url: &str) -> Result<String, io::Error> {
    /*
    download input from advent of code. be sure to set AOC_SESSION environment variable to your session cookie
     */
    let session_cookie = var("AOC_SESSION").expect("AOC_SESSION environment variable not set");
    match ureq::get(url).set("Cookie", &format!("session={}", session_cookie)).call() {
        Ok(response) => {
            response.into_string().map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to read response"))
        },
        Err(Error::Status(code, _response)) => {
            Err(io::Error::new(io::ErrorKind::Other, format!("Server returned status code: {}", code)))
        }
        Err(_) => {
            Err(io::Error::new(io::ErrorKind::Other, "Request failed"))
        }
    }
}
