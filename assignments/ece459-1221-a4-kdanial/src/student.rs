#[warn(unused_imports)]
// warn used to remove annoying warnings
use super::{checksum::Checksum, idea::Idea, package::Package, Event};
use crossbeam::channel::{Receiver, Sender};
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};

pub struct Student {
    id: usize,
    idea: Option<Idea>,
    pkgs: Vec<Package>,
    skipped_idea: bool,
    completed_recvr: Receiver<Event>,
    idea_recvr: Receiver<Idea>,
    package_recvr: Receiver<Event>,
    package_sender: Sender<Event>
}

impl Student {
<<<<<<< Updated upstream
    pub fn new(id: usize, completed_recvr: Receiver<Event>, idea_recvr: Receiver<Idea>, package_recvr: Receiver<Event>, package_sender: Sender<Event>) -> Self {
=======
    pub fn new(id: usize, idea_recvr: Receiver<Idea>, package_recvr: Receiver<Package>, package_sender: Sender<Package>) -> Self {
>>>>>>> Stashed changes
        Self {
            id,
            idea: None,
            pkgs: vec![],
            skipped_idea: false,
<<<<<<< Updated upstream
            completed_recvr,
=======
>>>>>>> Stashed changes
            idea_recvr,
            package_recvr,
            package_sender
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
                // We want the subsequent prints to be together, so we lock stdout
                let stdout = stdout();
                let mut handle = stdout.lock();
                /*
                writeln!(handle, "\nStudent {} built {} using {} packages\nIdea checksum: {}\nPackage checksum: {}",
                    self.id, idea.name, pkgs_required, idea_checksum, pkg_checksum).unwrap();
                for pkg in pkgs_used.iter() {
                    writeln!(handle, "> {}", pkg.name).unwrap();
                }
                */

                self.idea = None;
            }
        }
    }

    pub fn run(&mut self, idea_checksum: Arc<Mutex<Checksum>>, pkg_checksum: Arc<Mutex<Checksum>>) {
        loop {
            let received_idea = self.idea_recvr.try_recv();

            if  received_idea.is_ok() {
                self.idea = Some(received_idea.unwrap());
                
                loop {
                    let package = self.package_recvr.recv();

<<<<<<< Updated upstream
                    match package.unwrap() {
                        Event::DownloadComplete(package) => {
                            self.pkgs.push(package);
                            self.build_idea(&idea_checksum, &pkg_checksum);

                            if self.idea.is_none() {
                                break;
                            }
                        }
                        other => {}
                    }
                }

                // from starter code
                for package in self.pkgs.drain(..) {
                    self.package_sender.send(Event::DownloadComplete(package)).unwrap();
=======
                        // check if idea is not built
                        if self.idea == None {
                            break;
                        }
                    }

                    // from starter code
                    for package_to_send in self.pkgs.drain(..) {
                        self.package_sender.send(package_to_send).unwrap();
                    }
                },
                None => {
                    return;
>>>>>>> Stashed changes
                }
            }

            if self.idea_recvr.is_empty() && self.completed_recvr.try_recv().is_ok() { return; }
        }
    }
}
