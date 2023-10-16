use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut arr: Vec<char> = Vec::new();
        let mut visited: HashSet<char> = HashSet::new();
        let mut last_occurrence: HashMap<char, usize> = HashMap::new();

        for (i, character) in s.chars().enumerate() {
            last_occurrence.insert(character, i);
        }

        for (i, character) in s.chars().enumerate() {
            if !visited.contains(&character) {
                while !arr.is_empty()
                    && arr.last().unwrap() > &character
                    && last_occurrence.get(arr.last().unwrap()).unwrap() > &i
                {
                    visited.remove(&arr.pop().unwrap());
                }
                arr.push(character);
                visited.insert(character);
            }
        }

        arr.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let string = "bcabc".to_string();
        assert_eq!(
            Solution::remove_duplicate_letters(string),
            "abc".to_string()
        );
    }

    #[test]
    fn test_2() {
        let string = "cbacdcbc".to_string();
        assert_eq!(
            Solution::remove_duplicate_letters(string),
            "acdb".to_string()
        );
    }

    #[test]
    fn test_3() {
        // let string = "cbac".to_string(); // bac
        // let string = "cxyc".to_string(); // cxy
        let string = "caxc".to_string(); // axc
        assert_eq!(
            Solution::remove_duplicate_letters(string),
            "axc".to_string()
        );
    }
}
