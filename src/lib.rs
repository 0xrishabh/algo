mod search;
mod strings;

#[cfg(test)]
//use log::{info, warn}
mod tests {
    use super::*;

    #[test]
    fn linear_search_true() {
        let arr = vec![1, 2, 3, 4, 5];
        let key = 4;
        let success = search::linear_search(arr, key);
        assert_eq!(success, true);
    }
    #[test]
    fn linear_search_false() {
        let arr = vec![1, 2, 3, 4, 5];
        let key = 6;
        let success = search::linear_search(arr, key);
        assert_eq!(success, false);
    }
    #[test]
    fn binary_search_sucess() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let key = 5;
        let success = search::binary_search(arr, key);
        assert_eq!(success, true);
    }
    #[test]
    fn binary_search_failure() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let key = 20;
        let success = search::binary_search(arr, key);
        assert_eq!(success, false);
    }
    #[test]
    fn rabin_karp_true() {
        let pattern = String::from("is");
        let text = String::from("rishabh shukla is the best coder in the world");
        let success = strings::rabin_karp(pattern, text);
        assert_eq!(success, true);
    }
}
