pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let middle = array.len() / 2;

    if array[middle] == key {
        Some(middle)
    } else if array[middle] > key {
        find(&array[..middle], key)
    } else {
        find(&array[middle + 1..], key).map(|i| i + middle + 1)
    }
}
