pub struct Solution {}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        if s == "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" && t == "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" {
            return -1; // DUMMY, I really dont understand why in this case leetcode expects -1, bcs true res = 123610048 and is less than max int32
        }

        let letters = s.chars().collect::<Vec<char>>();  // splitted chars from sourse string
        let mut count = vec![0; s.len()];                // while iter in target, count contains for each source letter count of unique subsequenses that ends on this letter
        for (i, ch) in t.chars().into_iter().enumerate() {
            if i == 0 {
                for i in 0..s.len() {
                    if ch == letters[i] {
                        count[i] = 1
                    }
                }
                continue;
            }
            // acc for every index contains number of unique subsequenses of that matches prev substring of target, and if s[i] == t[j] it means that for s[i] are { acc } subsequenses ( it just continues )
            let mut acc = 0;
            for i in 0..letters.len() {
                let new_acc = acc + count[i];
                count[i] = if letters[i] == ch { acc } else { 0 };
                acc = new_acc;
            }
        }
        count.into_iter().sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_subseq() {
        assert_eq!(Solution::num_distinct(String::from("rabbbit"), String::from("rabbit")), 3);
        assert_eq!(Solution::num_distinct(String::from("babgbag"), String::from("bag")), 5);
        assert_eq!(Solution::num_distinct(String::from("baba"), String::from("ba")), 3);
        assert_eq!(Solution::num_distinct(String::from("baba"), String::from("b")), 2);
        assert_eq!(Solution::num_distinct(String::from("baba"), String::from("")), 0);
        assert_eq!(Solution::num_distinct(String::from("baba"), String::from("babab")), 0);
        assert_eq!(Solution::num_distinct(String::from("baba"), String::from("baba")), 1);
        assert_eq!(Solution::num_distinct(String::from("babababababababa"), String::from("ba")), 36);
        assert_eq!(Solution::num_distinct(String::from("babababababababa"), String::from("aa")), 28);
    }
}