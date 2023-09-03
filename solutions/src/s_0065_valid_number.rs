pub struct Solution {}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let (mut res, mut dot, mut exp_pos) = (false, false, None);
        for (i, c) in s.chars().enumerate() {
            match c {
                'e' | 'E' if exp_pos.is_none() && res => {
                    res = false;
                    dot = false;
                    exp_pos = Some(i);
                }
                '0'..='9' => res = true,
                '+' | '-' => match exp_pos {
                    Some(exp_pos) => {
                        if i != exp_pos + 1 {
                            return false;
                        }
                    }
                    _ => {
                        if i != 0 {
                            return false;
                        }
                    }
                },
                '.' if exp_pos.is_none() && !dot => dot = true,
                _ => return false,
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_valid_num() {
        assert_eq!(Solution::is_number(String::from("0")), true);
        assert_eq!(Solution::is_number(String::from(".1")), true);
        assert_eq!(Solution::is_number(String::from("1.")), true);
        assert_eq!(Solution::is_number(String::from("1.1")), true);

        assert_eq!(Solution::is_number(String::from("-.1")), true);
        assert_eq!(Solution::is_number(String::from("-1.")), true);
        assert_eq!(Solution::is_number(String::from("-1.1")), true);
        assert_eq!(Solution::is_number(String::from("-.1")), true);

        assert_eq!(Solution::is_number(String::from("")), false);
        assert_eq!(Solution::is_number(String::from("-+1")), false);
        assert_eq!(Solution::is_number(String::from("--1")), false);
        assert_eq!(Solution::is_number(String::from("++1")), false);
        assert_eq!(Solution::is_number(String::from("-")), false);
        assert_eq!(Solution::is_number(String::from("-1.1.1")), false);
        assert_eq!(Solution::is_number(String::from("1.1.1")), false);
    }

    #[test]
    fn test_exp_valid_num() {
        assert_eq!(Solution::is_number(String::from("-1e-1")), true);
        assert_eq!(Solution::is_number(String::from("1E1")), true);
        assert_eq!(Solution::is_number(String::from("1.21e1")), true);

        assert_eq!(Solution::is_number(String::from("0e")), false);
        assert_eq!(Solution::is_number(String::from("e1")), false);
        assert_eq!(Solution::is_number(String::from("-1e+-1")), false);
        assert_eq!(Solution::is_number(String::from("1.e1.21")), false);
        assert_eq!(Solution::is_number(String::from("1.e1e1")), false);
    }
}

/*
  A valid number can be split up into these components (in order):

  A decimal number or an integer.
  (Optional) An 'e' or 'E', followed by an integer.
  A decimal number can be split up into these components (in order):

  (Optional) A sign character (either '+' or '-').
  One of the following formats:
  One or more digits, followed by a dot '.'.
  One or more digits, followed by a dot '.', followed by one or more digits.
  A dot '.', followed by one or more digits.
  An integer can be split up into these components (in order):

  (Optional) A sign character (either '+' or '-').
  One or more digits.
  For example, all the following are valid numbers: ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1",
   "53.5e93", "-123.456e789"], while the following are not valid numbers: ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"].

  Given a string s, return true if s is a valid number.
*/
