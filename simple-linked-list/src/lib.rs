use std::iter::FromIterator;

// #[derive(Copy,Clone)]
struct Node<T> {
    data: T,                    // 值
    next: Option<Box<Node<T>>>, // 下一个节点
}
// impl<T> Node<T> {
//     fn new(data: T) -> Self {
//         Node { data, next: None }
//     }
// }
pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    dummy: ::std::marker::PhantomData<T>,

    head: Option<Box<Node<T>>>, // new->old
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            dummy: std::marker::PhantomData,
        }
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
        let mut cnt = 0;
        let mut p = &self.head;
        while let Some(node) = p {
            cnt += 1;
            p = &node.next
        }
        cnt
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                self.head = head.next;
                Some(head.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.data)
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        // 1->2->3
        // 3->2->1
        let mut new_head = SimpleLinkedList::new();
        while let Some(node) = self.pop() {
            new_head.push(node);
        }
        new_head
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut root = SimpleLinkedList::new();
        _iter.into_iter().for_each(|x| root.push(x));
        root
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

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut ans = Vec::new();
        let mut head = _linked_list.rev(); // 反转
        while let Some(node) = head.pop() {
            ans.push(node);
        }
        ans
    }
}
