pub struct Solution;

impl Solution {

    pub fn palindrome_checker(s:String,pos:usize) -> String {
        let limit_left = 0;
        let limit_right = s.len();
        let mut pos_left = pos - 1;
        let mut pos_right = pos + 1;
        loop {
            if s.chars().nth(pos_left) == s.chars().nth(pos_right) {
                if pos_left > limit_left && pos_right < limit_right {
                    pos_left -= 1;
                    pos_right += 1;
                }else {
                    return s.chars().skip(pos_left).take((pos_right-pos_left)+1).collect();
                }

            }else if s.chars().nth(pos_left) == s.chars().nth(pos) {
                return s.chars().skip(pos_left).take((pos-pos_left)+1).collect();
            }
            else if s.chars().nth(pos_right) == s.chars().nth(pos) {
                return s.chars().skip(pos).take((pos_right-pos)+1).collect();
            }
            else {
                pos_left += 1;
                pos_right -= 1;
                return s.chars().skip(pos_left).take((pos_right-pos_left)+1).collect();
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

