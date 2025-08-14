pub struct Solution;

impl Solution {

    pub fn palindrome_checker(s:String) -> String {
        let limit_left = 0;
        let limit_right = s.len();
        let pos = s.len()/2;
        let mut pos_left = pos - 1;
        let mut pos_right = pos + 1;
        loop {
            if s.chars().nth(pos_left) == s.chars().nth(pos_right) {
                if pos_left > limit_left {
                    pos_left -= 1;
                }

                if pos_right < limit_right {
                    pos_right += 1;
                }
            }else if s.chars().nth(pos_left) == s.chars().nth(pos) {

            }
            else if s.chars().nth(pos_right) == s.chars().nth(pos) {

            }
            else {
                return s[pos_left..pos_right].to_string() ;
            }
        }
    }

    pub fn longest_palindrome(s: String) {
        let limit_left = 0;
        let limit_right = s.len();
        let pos = s.len()/2;
        let pos_left = pos - 1;
        let pos_right = pos + 1;
        loop {

            
        }
    }
}

