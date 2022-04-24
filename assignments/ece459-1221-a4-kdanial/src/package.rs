use super::checksum::Checksum;
use super::Event;
use crossbeam::channel::Sender;
use std::fs;
use std::sync::{Arc, Mutex};

pub struct Package {
    pub name: String,
}

pub struct PackageDownloader {
    pkg_start_idx: usize,
    num_pkgs: usize,
    package_checksum: Checksum,
    package_sender: Sender<Event>,
}

impl PackageDownloader {
    pub fn new(pkg_start_idx: usize, num_pkgs: usize, package_sender: Sender<Event>) -> Self {
        Self {
            pkg_start_idx,
            num_pkgs,
            package_checksum: Checksum::default(),
            package_sender,
        }
    }

    pub fn run(&mut self, pkg_checksum: Arc<Mutex<Checksum>>) {
        // Generate a set of packages and place them into the event queue
        // Update the package checksum with each package name
        
        // Reference for line below: https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
        let package_names: Vec<String> = fs::read_to_string("data/packages.txt").unwrap().lines().map(|s| s.to_owned()).collect();

        for i in 0..self.num_pkgs {
            let package_index = (self.pkg_start_idx + i) % package_names.len();
            let name = package_names[package_index].clone();
            
            // modified starter code a bit
            self.package_checksum.update(Checksum::with_sha256(&name));
            self.package_sender.send(Event::DownloadComplete(Package { name })).unwrap();
        }

        pkg_checksum.lock().unwrap().update(self.package_checksum.clone());
    }
}
