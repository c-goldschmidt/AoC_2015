use crypto::md5::Md5;
use crypto::digest::Digest;

use crate::days::day::{New, Parts};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

pub struct Day4 {
    input: Vec<String>,
}

impl Day4 {

    fn check_chunk(&self, key: String, start:i32, chunk_size: i32, len: i32) -> i32 {
        let (tx, rx) = channel();
        let pool = ThreadPool::new(8);
        let len = len.clone();
        let key = key.into_bytes();

        for index in 0..chunk_size {
            let tx = tx.clone();
            let key = key.clone();
            let chunk_size = chunk_size.clone();

            pool.execute(move|| {
                let mut output = [0; 16]; // An MD5 is 16 bytes
                let mut hasher = Md5::new();
                let mut found_at = -1;

                for i in start..start + chunk_size {
                    let chunk_index = (index * chunk_size) + i;
                    hasher.input(&key);
                    hasher.input((chunk_index).to_string().as_bytes());
                    hasher.result(&mut output);
                    hasher.reset();

                    let hexdigest: Vec<String> = output.iter().map(|b| format!("{:02X}", b)).collect();
                    let connected = hexdigest.join("");

                    let mut found = true;
                    for ix in 0..len as usize {
                        if connected.as_bytes()[ix] != 48 {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        found_at = chunk_index;
                        break;
                    }
                }

                match tx.send(found_at) { _ => {}};
            });
        }

        for result in rx.iter().take(chunk_size as usize) {
            if result > 0 {
                return result;
            }
        }
        return -1;
    }

    fn find_hash(&self, len: i32) -> i32 {
        let mut result = -1;
        let mut start = 0;
        let chunk_size = 1000;

        while result < 0 {
            result = self.check_chunk(self.input[0].clone(), start, chunk_size, len);
            start += chunk_size * chunk_size;
        }

        return result;
    }
}

impl New<Day4> for Day4 {
    fn new(file_name: &str) -> Day4 {
        Day4 { input: Day4::get_content(file_name)}
    }
}

impl Parts for Day4 {
    fn part1(&self) -> i32 {
        return self.find_hash(5);
    }

    fn part2(&self) -> i32 {
        return self.find_hash(6);
    }
}