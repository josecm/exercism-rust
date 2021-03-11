use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Self>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<'a, T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut node = self.head.as_ref();
        let mut count = 0;
        while let Some(n) = node {
            count += 1;
            node = n.next.as_ref();
        }
        count
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(n) = self.head.take() {
            self.head = n.next;
            Some(n.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut rev = Self::new();
        while let Some(n) = self.pop() {
            rev.push(n)
        }
        rev
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for data in iter {
            list.push(data);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> 
    where T: Clone 
{
    fn into(self) -> Vec<T> {
       let rev = self.rev();
       let mut vec = Vec::new();
       let mut node = rev.head.as_ref();
       while let Some(n) = node {
           vec.push(n.data.clone());
            node = n.next.as_ref();
       }
       vec
    }
}
