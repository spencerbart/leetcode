struct Solution;
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let digits: Vec<i32> = digits.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        let len_of_digits = digits.len() as i32;
        let log = (n as f64).log10().floor() as u32;

        let mut sum: i32 = 0;

        // you can safely calculate all of the posibilities under the power of log.
        for i in 1..log+1 {
            sum += (len_of_digits).pow(i);
        }

        let n_as_array: Vec<u32> = n.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect();
        // now we need to calculate any possibilities in the given power.
        for i in n_as_array {
            for (j, d) in digits.iter().enumerate().rev() {
                // if n <= d {
                    
                // }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_0() {
        let digits = vec!["1","3","5","7"];
        let digits: Vec<String> = digits.iter().map(|x| x.to_string()).collect();
        let n = 100;

        let solution = Solution::at_most_n_given_digit_set(digits, n);
        assert_eq!(20, solution);
    }

    // #[test]
    // fn test_solution_1() {
    //     let digits = vec!["1","3","5","7"];
    //     let digits: Vec<String> = digits.iter().map(|x| x.to_string()).collect();
    //     let n = 99;

    //     let solution = Solution::at_most_n_given_digit_set(digits, n);
    //     assert_eq!(20, solution);
    // }
}