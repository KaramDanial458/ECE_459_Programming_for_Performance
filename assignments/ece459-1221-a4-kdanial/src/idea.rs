use super::checksum::Checksum;
use super::Event;
use crossbeam::channel::Sender;
use std::fs;
use std::sync::{Arc, Mutex};

pub struct Idea {
    pub name: String,
    pub num_pkg_required: usize,
}

pub struct IdeaGenerator {
    idea_start_idx: usize,
    num_ideas: usize,
    num_students: usize,
    num_pkgs: usize,
    pkg_per_idea: usize,
    extra_pkgs: usize,
    idea_checksum: Checksum,
    completed_sender: Sender<Event>,
    idea_sender: Sender<Idea>,
}

impl IdeaGenerator {
    pub fn new(
        idea_start_idx: usize,
        num_ideas: usize,
        num_students: usize,
        num_pkgs: usize,
        completed_sender: Sender<Event>,
        idea_sender: Sender<Idea>
    ) -> Self {
        Self {
            idea_start_idx,
            num_ideas,
            num_students,
            num_pkgs,
            pkg_per_idea: 0,
            extra_pkgs: 0,
            idea_checksum: Checksum::default(),
            completed_sender,
            idea_sender
        }
    }

    // Idea names are generated from cross products between product names and customer names
    fn get_next_idea_name(customers: String, products: String, idx: usize) -> String {
        let ideas = Self::cross_product(products, customers);
        let pair = &ideas[idx % ideas.len()];
        format!("{} for {}", pair.0, pair.1)
    }

    fn cross_product(products: String, customers: String) -> Vec<(String, String)> {
        products
            .lines()
            .flat_map(|p| customers.lines().map(move |c| (p.to_owned(), c.to_owned())))
            .collect()
    }

    pub fn run(&mut self, idea_checksum: Arc<Mutex<Checksum>>) {
        self.pkg_per_idea = self.num_pkgs / self.num_ideas;
        self.extra_pkgs = self.num_pkgs % self.num_ideas;

        let product_lines = fs::read_to_string("data/ideas-products.txt").expect("file not found");
        let customer_lines = fs::read_to_string("data/ideas-customers.txt").expect("file not found");

        // let products: String = product_lines.lines().map(|s| s.to_owned()).collect();
        // let customers: String = customer_lines.lines().map(|s| s.to_owned()).collect();

        // Generate a set of new ideas and place them into the event-queue
        // Update the idea checksum with all generated idea names
        for i in 0..self.num_ideas {
            let name = Self::get_next_idea_name(customer_lines.clone(), product_lines.clone(), self.idea_start_idx + i);
            let extra = (i < self.extra_pkgs) as usize;
            let num_pkg_required = self.pkg_per_idea + extra;
            let idea = Idea {
                name,
                num_pkg_required,
            };

            idea_checksum.lock().unwrap().update(Checksum::with_sha256(&idea.name));
            self.idea_sender.send(idea).unwrap();
        }

        // Push student termination events into the event queue
        for _ in 0..self.num_students {
            self.completed_sender.send(Event::OutOfIdeas).unwrap();
        }
    }
}
