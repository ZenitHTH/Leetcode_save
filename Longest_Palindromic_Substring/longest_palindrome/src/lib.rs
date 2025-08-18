pub struct Solution;

impl Solution {

    pub fn palindrome_checker(s:String,pos:usize) -> String {
        let limit_left = 0;
        let limit_right = s.len()-1;
        let mut pos_left = pos - 1;
        let mut pos_right = pos + 1;
        loop {
            if s.chars().nth(pos_left) == s.chars().nth(pos) && s.chars().nth(pos) != s.chars().nth(pos_right){
                //do it in case palindrome is even
                return Solution::palindrome_checker_even(pos_left, pos, s);
                //return s.chars().skip(pos_left).take((pos-pos_left)+1).collect();
            } else if s.chars().nth(pos_right) == s.chars().nth(pos) && s.chars().nth(pos_left) != s.chars().nth(pos) {
                //do it in case palindrome is even
                return Solution::palindrome_checker_even(pos, pos_right, s);
                //return s.chars().skip(pos).take((pos_right-pos)+1).collect();
            } else if s.chars().nth(pos_left) == s.chars().nth(pos_right) {
                if pos_left > limit_left && pos_right < limit_right {
                        pos_left -= 1;
                        pos_right += 1;
                    
                }else {
                    return s.chars().skip(pos_left).take((pos_right-pos_left)+1).collect();
                }
            }
            else {
                pos_left += 1;
                pos_right -= 1;
                return s.chars().skip(pos_left).take((pos_right-pos_left)+1).collect();
            }
        }
    }

    fn vec_get(pos_left:usize,pos_right:usize,v_str:Vec<char>) -> String {
        match v_str.get(pos_left..(pos_right+1)){
            Some(v) => {
                return v.iter().copied().collect::<String>();

            }
            None => {
                panic!("cannot get pos_left , pos_right");
            }
        }
    }

    fn palindrome_checker_even(left:usize,right:usize,s:String)->String {
        let mut pos_left: usize = left;
        let mut pos_right: usize = right;
        let limit_left:usize = 0;
        let limit_right:usize= s.len();
        let v_str:Vec<char> = s.chars().collect();
        loop {
            if v_str.get(pos_left) == v_str.get(pos_right) {
                if pos_left > limit_left && pos_right < limit_right {
                    pos_left -= 1;
                    pos_right +=1;
                }else{
                    return Solution::vec_get(pos_left,pos_right,v_str);
                }
            }else{
                pos_left +=  1;
                pos_right  -=1;
                return Solution::vec_get(pos_left,pos_right,v_str);
                // match v_str.get(pos_left..(pos_right+1)){
                //     Some(v) => {
                //         return v.iter().copied().collect::<String>();

                //     }
                //     None => {
                //         panic!("cannot get pos_left , pos_right");
                //     }
                // }
            }
        }
    }

    pub fn longest_palindrome(s: String) -> String {

        if s.len() > 2 {
            let mut vec_palindrome:Vec<String> = Vec::new();
            
            if s.len() > 3 {
                for pos in 1..s.len()-2 {
                    let res = self::Solution::palindrome_checker(s.clone(), pos);
                    if res.len() > 1 {
                        vec_palindrome.push(res);
                    }  
                }
            }else {
                // s.len() == 3
                vec_palindrome.push(self::Solution::palindrome_checker(s, 1));
            }   
            
            let mut longest:String = String::new();
            for v in vec_palindrome {
                if longest.len() < v.len() {
                    longest = v;
                }
            }
            return longest;
        }else {
            if s.len() <= 2 {
                let pos = 0;
                
                if s.chars().nth(pos) == s.chars().nth(pos+1) {
                    return s;
                }else {
                    match s.chars().nth(0) {
                        None => {
                            
                        }
                        Some(c)=>{
                            return c.to_string();
                        }
                    }
                }
            
            }
            return s;
        }
    }
}

