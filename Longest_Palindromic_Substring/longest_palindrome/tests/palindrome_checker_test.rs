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

    #[test]
    fn palindrome_checker_test7(){
        let s:String = String::from("ccc");
        let pos:usize = s.len()/2;
        let result:String = Solution::palindrome_checker(s, pos);
        assert_eq!(result,String::from("ccc"));
    }

    #[test]
    fn palindrome_checker_test8(){
        let s:String = String::from("aaaa");
        let pos:usize = s.len()/2;
        let result:String = Solution::palindrome_checker(s, pos);
        assert_eq!(result,String::from("aaaa"));
    }

    #[test]
    fn palindrome_checker9(){
        let s:String = String::from("rgdsooosgqw");
        let pos:usize = s.len()/2;
        let result:String = Solution::palindrome_checker(s, pos);
        assert_eq!(result,String::from("sooos"));
    }

    #[test]
    fn palindrome_checker10(){
        let s = String::from("aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa");
        let pos:usize = s.len()/2;
        let r = Solution::palindrome_checker(s, pos);
        assert_eq!(r,String::from("aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa"));
    }

    #[test]
    fn palindrome_checker11() {
        let s: String = String::from("caaaaa");
        let pos: usize = 3;
        let r: String = Solution::palindrome_checker(s, pos);
        assert_eq!(r,"aaaaa")
    }

    #[test]
    fn palindrome_checker12() {
        let s: String = String::from("caaaaa");
        let pos: usize = 4;
        let r: String = Solution::palindrome_checker(s, pos);
        assert_eq!(r,"aaa")
    } 
    

}
