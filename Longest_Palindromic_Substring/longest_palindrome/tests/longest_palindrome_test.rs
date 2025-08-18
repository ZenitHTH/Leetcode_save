#[cfg(test)]
mod tests {
    use longest_palindrome::Solution;

    #[test]
    fn longest_palindrome_test1(){
        let s:String = String::from("a");
        let r = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("a"));
    }

    #[test]
    fn longest_palindrome_test2(){
        let s:String = String::from("ac");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("a"));
    }

    #[test]
    fn longest_palindrome_test3(){
        let s:String = String::from("babad");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("aba"));
    }

    #[test]
    fn longest_palindrome_test4(){
        let s:String = String::from("cbbd");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("bb"));
    }

    #[test]
    fn longest_palindrome_test5(){
        let s:String = String::from("bb");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("bb"));
    }

}