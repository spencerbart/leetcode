struct Solution;
impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let mut output = 0;
        
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr: Vec<i32> = vec![10,13,12,14,15];
        let solution = Solution::odd_even_jumps(arr);
        assert_eq!(solution, 2);
    }
    #[test]
    fn test_2() {
        let arr: Vec<i32> = vec![2,3,1,1,4];
        let solution = Solution::odd_even_jumps(arr);
        assert_eq!(solution, 3);
    }
    #[test]
    fn test_3() {
        let arr: Vec<i32> = vec![5,1,3,4,2];
        let solution = Solution::odd_even_jumps(arr);
        assert_eq!(solution, 3);
    }
}