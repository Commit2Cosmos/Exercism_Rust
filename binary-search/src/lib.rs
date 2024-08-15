pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    
    let mut left = 0;
    let mut right = array.len() - 1;
    let mut mid = (right + left) / 2;

    while left < right {
        if key > array[mid] {
            left = mid+1;
        } else {
            right = mid;
        }
        mid = (right + left) / 2;
    }

    if array[mid] == key {
        Some(mid as usize)
    } else {
        None
    }
}
