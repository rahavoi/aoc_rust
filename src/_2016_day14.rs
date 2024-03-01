use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;
use std::hash::Hash;

//Assumption: ASCII chars only
pub fn solve(input: &str) {
    let mut count_valid_keys = 0;
    let mut idx = 0;
    let mut memory: HashMap<String, String> = HashMap::new();

    while count_valid_keys < 64 {
        let hash_binding = input.hash_suffix_2016(idx.clone(), &mut memory);
        let hash = hash_binding.as_str();

        match hash.get_first_byte_repeating_3_times_in_a_row() {
            Some(c) => {
                for i in 1..1000 {
                    let sub_idx = &idx + i;

                    let binding = input.hash_suffix_2016(sub_idx.clone(), &mut memory);
                    let test_hash = binding.as_str();

                    if test_hash.has_same_byte_5_times_in_a_row(c.clone()) {
                        println!(
                            "{}. {}. Repeating 5 times: {}. Index: {}",
                            count_valid_keys, test_hash, c as char, &idx
                        );
                        count_valid_keys = count_valid_keys + 1;
                        break;
                    }
                }
            }
            None => (),
        }

        idx = idx + 1;
    }
}

pub trait HashUtils {
    fn hash_suffix(&self, suffix: usize) -> String;
    fn hash_suffix_2016(&self, suffix: usize, memory: &mut HashMap<String, String>) -> String;
    fn has_same_byte_5_times_in_a_row(&self, c: u8) -> bool;
    fn get_first_byte_repeating_3_times_in_a_row(&self) -> Option<u8>;
}

impl HashUtils for &str {
    fn hash_suffix(&self, suffix: usize) -> String {
        let suffixed = format!("{}{}", self, suffix);
        let result = md5::compute(&suffixed);
        format!("{:x}", result)
    }
    fn hash_suffix_2016(&self, suffix: usize, memory: &mut HashMap<String, String>) -> String {
        let suffixed = format!("{}{}", self, suffix);

        if memory.contains_key(&suffixed) {
            return memory.get(&suffixed).unwrap().to_string();
        }

        let mut result = format!("{:x}", md5::compute(&suffixed));

        for _ in 0..2016 {
            result = format!("{:x}", md5::compute(&result));
        }

        memory.insert(suffixed, result.clone());
        result
    }

    fn has_same_byte_5_times_in_a_row(&self, c: u8) -> bool {
        let bytes = self.as_bytes();

        for i in 4..bytes.len() {
            if bytes[i] == c
                && bytes[i] == bytes[i - 1]
                && bytes[i] == bytes[i - 2]
                && bytes[i] == bytes[i - 3]
                && bytes[i] == bytes[i - 4]
            {
                return true;
            }
        }

        return false;
    }

    fn get_first_byte_repeating_3_times_in_a_row(&self) -> Option<u8> {
        let mut hash_bytes = self.as_bytes();

        //md5 is always 32 characters long
        for i in 2..32 {
            if hash_bytes[i] == hash_bytes[i - 1] && hash_bytes[i] == hash_bytes[i - 2] {
                return Some(hash_bytes[i].clone());
            }
        }

        None
    }
}
