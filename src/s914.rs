use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut frequency_map = HashMap::new();
        for i in &deck {
            let count = frequency_map.entry(i).or_insert(0);
            *count += 1;
        }
        let frequency: Vec<i32> = frequency_map.values().cloned().collect();

        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let mut common_gcd = frequency[0];
        for val in frequency.iter().skip(1) {
            common_gcd = gcd(common_gcd, *val);

            if common_gcd == 1 {
                return false;
            }
        }
        common_gcd > 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let dec = vec![1, 2, 3, 4, 4, 3, 2, 1];
        let solution = Solution::has_groups_size_x(dec);
        assert_eq!(solution, true);
    }

    #[test]
    fn test_solution_2() {
        let dec = vec![1, 1, 1, 2, 2, 2, 3, 3];
        let solution = Solution::has_groups_size_x(dec);
        assert_eq!(solution, false);
    }

    #[test]
    fn test_solution_3() {
        let dec = vec![1];
        let solution = Solution::has_groups_size_x(dec);
        assert_eq!(solution, false);
    }

    #[test]
    fn test_solution_4() {
        let dec = vec![1, 1, 2, 2, 2, 2];
        let solution = Solution::has_groups_size_x(dec);
        assert_eq!(solution, true);
    }

    #[test]
    fn test_solution_5() {
        let dec = vec![1, 1, 1, 1, 2, 2, 2, 2, 2, 2];
        let solution = Solution::has_groups_size_x(dec);
        assert_eq!(solution, true);
    }

    #[test]
    fn test_solution_6() {
        // [6,9,12]
        let dec = vec![
            1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        ];
        let solution = Solution::has_groups_size_x(dec);
        assert_eq!(solution, true);
    }
}
