use std::cmp::Ordering;

pub fn find<T, U>(array: U, key: T) -> Option<usize>
where
    T: Ord,
    U: AsRef<[T]>,
{
    let array = array.as_ref();
    let middle = array.len() / 2;
    match key.cmp(array.get(middle)?) {
        Ordering::Equal => Some(middle),
        Ordering::Less => find(&array[..middle], key),
        Ordering::Greater => find(&array[middle + 1..], key).map(|i| i + middle + 1),
    }
}
