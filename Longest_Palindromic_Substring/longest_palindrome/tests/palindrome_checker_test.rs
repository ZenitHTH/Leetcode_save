#[cfg(test)]
mod tests {
    use longest_palindrome::Solution;
    
    #[test]
    fn palindrome_checker_test1() {
        let result = Solution::palindrome_checker(String::from("bcdcb"));
        assert_eq!(result,String::from("bcdcb"));
    }

    #[test]
    fn palindrome_checker_test2(){
        let result = Solution::palindrome_checker(String::from("abcdcfg"));
        assert_eq!(result,String::from("cdc"));
    }

    #[test]
    fn palindrome_checker_test3(){
        let result:String = Solution::palindrome_checker(String::from("abcddef"));
        assert_eq!(result,String::from("dd"))
    }

    #[test]
    fn palindrome_checker_test4(){
        let result:String = Solution::palindrome_checker(String::from("abccdef"));
        assert_eq!(result,String::from("cc"));
    }


}
