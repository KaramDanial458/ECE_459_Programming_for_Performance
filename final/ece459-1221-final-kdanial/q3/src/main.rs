use std::thread;
use core::cmp::max;
use std::collections::HashSet;
use std::collections::HashMap;
use std::env;

// The character 'a' is represented by ASCII code 97. If you want to treat
// 'a' as index 0 of the alphabet, the offset here helps.
const ASCII_OFFSET: usize = 97;
const NUM_THREADS: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];

    let words = read_words_from_file(filename);
    println!("There are {} words to evaluate.", words.len());

    let letter_frequency = letter_frequency(&words);
    let letter_frequency = rank(letter_frequency);

    let word_score = score_words(&words, letter_frequency);

    let max = find_max_score(&word_score);
    println!("Max score is: {}.", max);
    let max_words = find_max_words(word_score, &max);

    print_suggestions(max_words);
    println!();
}

fn letter_frequency(words: &[String]) -> Vec<i32> {
    // TODO: Implement!
    let thread_partition: usize  = max(words.len() / NUM_THREADS, 1);
    let mut handlers = Vec::new();

    for i in 0..NUM_THREADS {
        let words_2 = words.to_vec();

        let handle = thread::spawn(move || {
            let mut result: Vec<i32> = vec![0; 26];
            let starting_point = i * thread_partition;
            let ending_point = starting_point + thread_partition;

            for index in starting_point..ending_point {
                if index >= words_2.len() {
                    return result;
                } else {
                    for c in words_2[index].chars() {
                        let tmp_char_index: usize = c as usize;
                        result[tmp_char_index - ASCII_OFFSET] += 1;
                    }
                }
            }

            return result;
        });

        handlers.push(handle);
    }

    let mut answer: Vec<i32> = vec![0; 26];

    for handler in handlers {
        let tmp_freq_array = handler.join().unwrap();

        for i in 0..26 {
            answer[i] += tmp_freq_array[i];
        }
    }
    answer
}

fn score_words(words: &[String], frequency: Vec<i32>) -> HashMap<String, i32> {
    // TODO: Implement!
    let thread_partition: usize  = max(words.len() / NUM_THREADS, 1);
    let mut handlers = Vec::new();

    for i in 0..NUM_THREADS {
        let words_2 = words.to_vec();
        let frequency_2 = frequency.clone();
        let mut result: HashMap<String, i32> = HashMap::new();

        let handle = thread::spawn(move || {
            let starting_point = i * thread_partition;
            let ending_point = starting_point + thread_partition;

            for index in starting_point..ending_point {
                let mut visited_set: HashSet<char> = HashSet::new();

                if index >= words_2.len() {
                    return result;
                }

                for c in words_2[index].chars() {
                    if !visited_set.contains(&c) {
                        visited_set.insert(c);
                    }
                }

                for letter in &visited_set {
                    if !result.contains_key(&words_2[index]) {
                        result.insert(words_2[index].clone(), 0);
                    }
                    *result.get_mut(&words_2[index]).unwrap() += frequency_2[*letter as usize - ASCII_OFFSET];
                }
            }

            return result;
        });

        handlers.push(handle);
    }

    let mut answer: HashMap<String, i32> = HashMap::new();

    for handler in handlers {
        let new_map = handler.join().unwrap();
        answer.extend(new_map.into_iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    answer
}

fn find_max_score(word_score: &HashMap<String, i32>) -> i32 {
    let mut max = 0;
    for (_word, score) in word_score.iter() {
        if *score > max {
            max = *score;
        }
    }
    max
}

fn find_max_words(word_score: HashMap<String, i32>, max: &i32) -> Vec<String> {
    let mut max_words = Vec::new();
    for (word, score) in word_score.iter() {
        if *score == *max {
            max_words.push(word.clone());
        }
    }
    max_words
}

fn print_suggestions(max_words: Vec<String>) {
    print!("Suggestion(s): ");
    let mut first = true;
    for item in max_words {
        if !first {
            print!(", ");
        }
        print!("{}", item);
        first = false;
    }
}

fn read_words_from_file(inputfilename: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut rdr = csv::Reader::from_path(inputfilename).unwrap();
    for line in rdr.records() {
        let word = String::from(line.unwrap().get(0).unwrap());
        if word.len() > 5 {
            panic!("Word too long: {}", word);
        }
        words.push(word);
    }
    words
}

fn rank(frequency: Vec<i32>) -> Vec<i32> {
    let mut ranks = frequency.clone();
    ranks.sort_unstable();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut index = 1;
    let mut prev = *ranks.get(0).unwrap();
    map.insert(prev, index);

    for i in 1..frequency.len() {
        let cur = ranks.get(i).unwrap();
        if prev != *cur {
            index += 1;
        }
        map.insert(*cur, index);
        prev = *cur
    }

    for j in 0..frequency.len() {
        ranks[j] = *map.get(&frequency[j]).unwrap();
    }
    ranks
}
