pub fn find<T, U>(array: U, key: T) -> Option<usize>
where
    T: PartialOrd,
    U: AsRef<[T]>,
{
    let array = array.as_ref();

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
