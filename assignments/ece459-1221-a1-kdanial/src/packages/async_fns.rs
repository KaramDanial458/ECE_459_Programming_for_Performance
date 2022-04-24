use urlencoding::encode;
use curl::Error;
use curl::easy::{Easy2, Handler, WriteError};
use curl::multi::{Easy2Handle, Multi};
use std::collections::HashMap;
use std::time::Duration;
use std::str;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::Packages;

struct Collector(Box<String>);
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        (*self.0).push_str(str::from_utf8(&data.to_vec()).unwrap());
        Ok(data.len())
    }
}

const DEFAULT_SERVER : &str = "ece459.patricklam.ca:4590";
impl Drop for Packages {
    fn drop(&mut self) {
        self.execute()
    }
}

static EASYKEY_COUNTER: AtomicI32 = AtomicI32::new(0);

pub struct AsyncState {
    server : String,
    urls : Vec<Vec<String>>
}

impl AsyncState {
    pub fn new() -> AsyncState {
        AsyncState {
            server : String::from(DEFAULT_SERVER),
            urls : Vec::new()
        }
    }
}

// From Lecture 5
fn init(multi:&Multi, url:&str) -> Result<Easy2Handle<Collector>, Error> {
    let mut easy = Easy2::new(Collector(Box::new(String::new())));
    easy.url(url)?;
    easy.verbose(false)?;
    Ok(multi.add2(easy).unwrap())
}

impl Packages {
    pub fn set_server(&mut self, new_server:&str) {
        self.async_state.server = String::from(new_server);
    }

    /// Retrieves the version number of pkg and calls enq_verify_with_version with that version number.
    pub fn enq_verify(&mut self, pkg:&str) {
        let version = self.get_available_debver(pkg);
        match version {
            None => { println!("Error: package {} not defined.", pkg); return },
            Some(v) => { 
                let vs = &v.to_string();
                self.enq_verify_with_version(pkg, vs); 
            }
        };
    }

    /// Enqueues a request for the provided version/package information. Stores any needed state to async_state so that execute() can handle the results and print out needed output.
    pub fn enq_verify_with_version(&mut self, pkg:&str, version:&str) {
        let url = format!("http://{}/rest/v1/checksums/{}/{}", DEFAULT_SERVER, pkg, version);
        println!("queueing request {}", url);
        let mut tmp_arr: Vec<String> = Vec::new();
        tmp_arr.push(pkg.to_string());
        tmp_arr.push(version.to_string());
        tmp_arr.push(url);
        self.async_state.urls.push(tmp_arr);
    }

    /// Asks curl to perform all enqueued requests. For requests that succeed with response code 200, compares received MD5sum with local MD5sum (perhaps stored earlier). For requests that fail with 400+, prints error message.
    pub fn execute(&mut self) {
        let mut easys : Vec<Easy2Handle<Collector>> = Vec::new();
        let mut multi = Multi::new();

        multi.pipelining(true, true).unwrap();

        for vector in self.async_state.urls.iter() {
            easys.push(init(&multi, &vector[2]).unwrap());
        }

        while multi.perform().unwrap() > 0 {
            multi.wait(&mut [], Duration::from_secs(30)).unwrap();
        }

        let mut url_index = 0;
        for eh in easys.drain(..) {
            let mut handler_after:Easy2<Collector> = multi.remove2(eh).unwrap();
            let response = handler_after.response_code().unwrap();
            let result = handler_after.get_ref().0.to_string();

            let pkg_name = &self.async_state.urls[url_index][0];
            let pkg_vers = &self.async_state.urls[url_index][1];

            if response == 200 {
                println!("verifying {}, matches: {:?}", pkg_name, self.md5sums[&self.package_name_to_num[&self.async_state.urls[url_index][0]]].eq(&result));

            } else {
                println!("got error {} on request for package {} version {}", response, pkg_name, pkg_vers);
            }
            url_index += 1;
        }
        self.async_state.urls = Vec::new();
    }
}
