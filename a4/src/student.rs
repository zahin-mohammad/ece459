use super::{checksum::Checksum, idea::Idea, package::Package, Event};
use crossbeam::channel::{Receiver, Sender};
use std::{sync::{Arc, Mutex}};

pub struct Student {
    idea: Option<Idea>,
    pkgs: Vec<Package>,
    recv_idea: Receiver<Idea>,
    send_download: Sender<Package>,
    recv_download: Receiver<Package>,
    recv_done: Receiver<Event>,
}

impl Student {
    pub fn new(
        recv_idea: Receiver<Idea>,
        send_download: Sender<Package>,
        recv_download: Receiver<Package>,
        recv_done: Receiver<Event>,
    ) -> Self {
        Self {
            idea: None,
            pkgs: vec![],
            recv_idea,
            send_download,
            recv_download,
            recv_done
        }
    }

    fn build_idea(
        &mut self,
        idea_checksum: &Arc<Mutex<Checksum>>,
        pkg_checksum: &Arc<Mutex<Checksum>>,
    ) {
        if let Some(ref idea) = self.idea {
            // Can only build ideas if we have acquired sufficient packages
            let pkgs_required = idea.num_pkg_required;
            if pkgs_required <= self.pkgs.len() {
                let (mut idea_checksum, mut pkg_checksum) =
                    (idea_checksum.lock().unwrap(), pkg_checksum.lock().unwrap());

                // Update idea and package checksums
                // All of the packages used in the update are deleted, along with the idea
                idea_checksum.update(Checksum::with_sha256(&idea.name));
                let pkgs_used = self.pkgs.drain(0..pkgs_required).collect::<Vec<_>>();
                for pkg in pkgs_used.iter() {
                    pkg_checksum.update(Checksum::with_sha256(&pkg.name));
                }
                self.idea = None;
            }
        }
    }

    pub fn run(&mut self, idea_checksum: Arc<Mutex<Checksum>>, pkg_checksum: Arc<Mutex<Checksum>>) {
        loop {
            let idea = self.recv_idea.try_recv();
            if idea.is_ok() {
                self.idea = Some(idea.unwrap());
                // Download
                loop {
                    let pkg: Package = self.recv_download.recv().unwrap();
                    self.pkgs.push(pkg);
                    self.build_idea(&idea_checksum, &pkg_checksum);
                    if self.idea.is_none() {
                        break;
                    }
                }
                // Send un-used packages
                for pkg in self.pkgs.drain(..) {
                    self.send_download
                        .send(pkg)
                        .unwrap();
                }
            }
            // Out of ideas
            if self.recv_idea.is_empty() && self.recv_done.try_recv().is_ok() {
                return;
            } 
        }
    }
}
