pub fn binary_search(arr: Vec<i32>, key: i32) -> bool {
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        if arr[mid] == key {
            return true;
        } else {
            if key > arr[mid] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }
    return false;
}

pub fn linear_search(arr: Vec<i32>, key: i32) -> bool {
    for ele in &arr {
        if ele == &key {
            return true;
        }
    }
    return false;
}
