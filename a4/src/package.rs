use super::checksum::Checksum;
use crossbeam::channel::Sender;
use std::fs;
use std::sync::{Arc, Mutex};

pub struct Package {
    pub name: String,
}

pub struct PackageDownloader {
    pkg_start_idx: usize,
    num_pkgs: usize,
    send_download: Sender<Package>,
}

impl PackageDownloader {
    pub fn new(pkg_start_idx: usize, num_pkgs: usize, send_download: Sender<Package>) -> Self {
        Self {
            pkg_start_idx,
            num_pkgs,
            send_download,
        }
    }

    pub fn run(&self, pkg_checksum: Arc<Mutex<Checksum>>) {
        // Generate a set of packages and place them into the event queue
        // Update the package checksum with each package name
        let name: Vec<String> = fs::read_to_string("data/packages.txt")
            .unwrap()
            .lines()
            .map(|l| l.to_owned())
            .collect::<Vec<String>>();
        let mut local_checksum = Checksum::default();
        for i in 0..self.num_pkgs {
            let name = &name[(self.pkg_start_idx + i) % name.len()];
            local_checksum
                .update(Checksum::with_sha256(&name));
            self.send_download
                .send(Package { 
                    name: name.to_owned() 
                })
                .unwrap();
        }
        pkg_checksum
            .lock()
            .unwrap()
            .update(local_checksum);
    }
}
