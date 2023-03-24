mod search;
#[cfg(test)]
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
}
