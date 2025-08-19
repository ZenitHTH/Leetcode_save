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
        assert_eq!(r,String::from("bab"));
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

    #[test]
    fn longest_palindrome_test6(){
        let s:String = String::from("ccc");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("ccc"));
    }


    #[test]
    fn longest_palindrome_test7(){
        let s:String = String::from("aaaa");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("aaaa"));
    }

    #[test]
    fn longest_palindrome_test8(){
        let s:String = String::from("acbaabed");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("baab"))
    }

    #[test]
    fn longest_palindrome_test9(){
        let s:String = String::from("abb");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("bb"));
    }

    #[test]
    fn longest_palindrome_test10(){
        let s:String = String::from("caba");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("aba"));
    }

    #[test]
    fn longest_palindrome_test11(){
        let s:String = String::from("xaabacxcabaaxcabaax");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("xaabacxcabaax"));
    }

    #[test]
    fn longest_palindrome_test12(){
        let s:String = String::from("abcda");
        let r:String = Solution::longest_palindrome(s);
        assert_eq!(r,String::from("a"));
    }

}