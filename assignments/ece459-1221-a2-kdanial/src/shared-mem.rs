// Starter code for ECE 459 Lab 2, Winter 2021

// YOU SHOULD MODIFY THIS FILE TO USE THREADING AND SHARED MEMORY

#![warn(clippy::all)]
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::env;

use std::thread;
use std::sync::{Mutex, Arc};
use std::thread::sleep;
use std::time::Duration;
use crossbeam::epoch::Shared;

const DEFAULT_ALPHABETS: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

type HmacSha256 = Hmac<Sha256>;

// Check if a JWT secret is correct
fn is_secret_valid(msg: &[u8], sig: &[u8], secret: &[u8]) -> bool {
    let mut mac = HmacSha256::new_varkey(secret).unwrap();
    mac.update(msg);
    mac.verify(sig).is_ok()
}

struct SharedBuffer {
    buffer: Vec<u8>
}

// Contextual info for solving a JWT
#[derive(Clone)]
struct JwtSolver {
    alphabet: Vec<u8>, // set of possible bytes in the secret
    max_len: usize,    // max length of the secret
    msg: Vec<u8>,      // JWT message
    sig64: Vec<u8>,    // JWT signature (base64 decoded)
}

impl JwtSolver {
    // Recursively check every possible secret string,
    // returning the correct secret if it exists
    fn check_all(&self, secret: Vec<u8>) -> Option<Vec<u8>> {
        if is_secret_valid(&self.msg, &self.sig64, &secret) {
            return Some(secret);  // found it!
        }

        if secret.len() == self.max_len {
            return None;  // no secret of length <= max_len
        }

        for &c in self.alphabet.iter() {
            // allocate space for a secret one character longer  
            let mut new_secret = Vec::with_capacity(secret.len() + 1);
            // build the new secret
            new_secret.extend(secret.iter().chain(&mut [c].iter()));
            // check this secret, and recursively check longer ones
            if let Some(ans) = self.check_all(new_secret) {
                return Some(ans);
            }
        }
        None
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("Usage: <token> <max_len> [alphabet]");
        return;
    }

    let token = &args[1];

    let max_len = match args[2].parse::<u32>() {
        Ok(len) => len,
        Err(_) => {
            eprintln!("Invalid max length");
            return;
        }
    };

    let alphabet = args
        .get(3)
        .map(|a| a.as_bytes())
        .unwrap_or(DEFAULT_ALPHABETS)
        .into();

    // find index of last '.'
    let dot = match token.rfind('.') {
        Some(pos) => pos,
        None => {
            eprintln!("No dot found in token");
            return;
        }
    };

    // message is everything before the last dot
    let msg = token.as_bytes()[..dot].to_vec();
    // signature is everything after the last dot
    let sig = &token.as_bytes()[dot + 1..];

    // convert base64 encoding into binary
    let sig64 = match base64::decode_config(sig, base64::URL_SAFE_NO_PAD) {
        Ok(sig) => sig,
        Err(_) => {
            eprintln!("Invalid signature");
            return;
        }
    };

    // build the solver and run it to get the answer
    let solver = JwtSolver {
        alphabet,
        max_len: max_len as usize,
        msg,
        sig64,
    };

    let num_threads = num_cpus::get();
    let thread_char_groups: Vec<Vec<u8>> = solver.alphabet.clone().chunks((solver.alphabet.len() / num_threads)
        .max(1)).map(|c| c.into()).collect();

    let mut handles = Vec::new();

    // Start shared memory
    // let shared_buf = SharedBuffer {
    //     buffer: vec![30]
    // };
    let buffer: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));

    for char_group in thread_char_groups {
        let current_group = char_group;
        let solver_clone = solver.clone();
        let buffer = buffer.clone();

        let handle = thread::spawn(move || {
            for &char in current_group.iter() {
                // Check if answer in shared memory
                let mut buf = buffer.lock().unwrap();
                if *buf != None {
                    return;
                }

                let tmp_vec = vec![char];
                let ans = solver_clone.check_all(tmp_vec);
                if ans == None {
                    continue;
                } else {
                    // Send to shared buffer
                    let ans_string = ans.unwrap();
                    *buf = Option::from(ans_string);
                    return;
                }
            }
            // In case there are no answers are found get rid of hanging threads
            return;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join();
    }

    // Get answer from main thread
    let answer = Arc::try_unwrap(buffer).unwrap();
    let ans = answer.into_inner().unwrap();

    match ans {
        Some(ans) => println!(
            "{}", std::str::from_utf8(&ans).expect("answer not a valid string")
        ),
        None => println!("No answer found"),
    };
}
