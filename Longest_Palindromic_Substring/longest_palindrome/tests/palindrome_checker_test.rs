#[cfg(test)]
mod tests {
    use longest_palindrome::Solution;
    
    #[test]
    fn palindrome_checker_test1() {
        let s:String = String::from("bcdcb");
        let pos:usize = s.len()/2;
        let result = Solution::palindrome_checker(s,pos);
        assert_eq!(result,String::from("bcdcb"));
    }

    #[test]
    fn palindrome_checker_test2(){
        let s:String = String::from("abcdcfg");
        let pos:usize = s.len()/2;
        let result = Solution::palindrome_checker(s,pos);
        assert_eq!(result,String::from("cdc"));
    }

    #[test]
    fn palindrome_checker_test3(){
        let s:String = String::from("abcddef");
        let pos:usize = s.len()/2;
        let result:String = Solution::palindrome_checker(s,pos);
        assert_eq!(result,String::from("dd"))
    }

    #[test]
    fn palindrome_checker_test4(){
        let s:String = String::from("abccdef");
        let pos:usize = s.len()/2;
        let result:String = Solution::palindrome_checker(s,pos);
        assert_eq!(result,String::from("cc"));
    }

    #[test]
    fn palindrome_checker_test5(){
        let s:String = String::from("abcdef");
        let pos:usize = s.len()/2;
        let result:String = Solution::palindrome_checker(s, pos);
        assert_eq!(result.len(),1);
    }

    #[test]
    fn palindrome_checker_test6(){
        let s:String = String::from("abcdcf");
        let pos:usize = s.len()/2;
        let result:String = Solution::palindrome_checker(s, pos);
        assert_eq!(result,String::from("cdc"));
    }



}
