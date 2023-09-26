struct Solution;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut arr: Vec<char> = vec![];

        for character in s.chars() {
            // find the index of the character in arr if it exists
            let mut index = None;
            for (i, n) in arr.iter().enumerate() {
                if character == *n {
                    index = Some(i);
                    break;
                }
            }

            if index.is_some() {
                for n in arr.iter().skip(index.unwrap()) {
                   if character > *n {
                       arr.remove(index.unwrap());
                       arr.push(character);
                       break;
                   }
                }
            } else {
                arr.push(character);
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
        assert_eq!(Solution::remove_duplicate_letters(string), "abc".to_string());
    }

    #[test]
    fn test_2() {
        let string = "cbacdcbc".to_string();
        assert_eq!(Solution::remove_duplicate_letters(string), "acbd".to_string());
    }

    #[test]
    fn test_3() {
        let string = "cbac".to_string(); // bac
        let string = "cxyc".to_string(); // cxy
        let string = "caxc".to_string(); // axc
        assert_eq!(Solution::remove_duplicate_letters(string), "axc".to_string());
    }
}