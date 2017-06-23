use std::sync::Arc;
use std::iter::FromIterator;
use conslist::ConsList;

/// A strict queue backed by a pair of linked lists.
///
/// All operations run in O(1) amortised time, but the `pop`
/// operation may run in O(n) time in the worst case.
pub struct Queue<A>(ConsList<A>, ConsList<A>);

impl<A> Queue<A> {
    /// Construct an empty queue.
    pub fn new() -> Self {
        Queue(conslist![], conslist![])
    }

    /// Test whether a queue is empty.
    ///
    /// Time: O(1)
    pub fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    /// Get the length of a queue.
    ///
    /// Time: O(1)
    pub fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    /// Construct a new queue by appending an element to the end
    /// of the current queue.
    ///
    /// Time: O(1)
    pub fn push<R>(&self, v: R) -> Self
    where
        Arc<A>: From<R>,
    {
        Queue(self.0.clone(), self.1.cons(v))
    }

    /// Get the first element out of a queue, as well as the remainder
    /// of the queue.
    ///
    /// Returns `None` if the queue is empty. Otherwise, you get a tuple
    /// of the first element and the remainder of the queue.
    pub fn pop(&self) -> Option<(Arc<A>, Queue<A>)> {
        match self {
            &Queue(ref l, ref r) if l.is_empty() && r.is_empty() => None,
            &Queue(ref l, ref r) => {
                match l.uncons() {
                    None => Queue(r.reverse(), conslist![]).pop(),
                    Some((a, d)) => Some((a, Queue(d, r.clone()))),
                }
            }
        }
    }

    /// Get an iterator over a queue.
    pub fn iter(&self) -> Iter<A> {
        Iter { current: self.clone() }
    }
}

impl<A> Clone for Queue<A> {
    fn clone(&self) -> Self {
        Queue(self.0.clone(), self.1.clone())
    }
}

/// An iterator over a queue of elements of type `A`.
pub struct Iter<A> {
    current: Queue<A>,
}

impl<A> Iterator for Iter<A> {
    type Item = Arc<A>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.pop() {
            None => None,
            Some((a, q)) => {
                self.current = q;
                Some(a)
            }
        }
    }
}

impl<A> IntoIterator for Queue<A> {
    type Item = Arc<A>;
    type IntoIter = Iter<A>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { current: self }
    }
}

impl<'a, A> IntoIterator for &'a Queue<A> {
    type Item = Arc<A>;
    type IntoIter = Iter<A>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<A, T> FromIterator<T> for Queue<A>
where
    Arc<A>: From<T>,
{
    fn from_iter<I>(source: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        source.into_iter().fold(Queue::new(), |q, v| q.push(v))
    }
}

// QuickCheck

#[cfg(any(test, feature = "quickcheck"))]
use quickcheck::{Arbitrary, Gen};

#[cfg(any(test, feature = "quickcheck"))]
impl<A: Arbitrary + Sync> Arbitrary for Queue<A> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Queue::from_iter(Vec::<A>::arbitrary(g))
    }
}

// Tests

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn general_consistency() {
        let q = Queue::new().push(1).push(2).push(3).push(4).push(5).push(6);
        assert_eq!(6, q.len());
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(vec, Vec::from_iter(q.iter().map(|a| *a)))
    }

    quickcheck! {
        fn length(v: Vec<i32>) -> bool {
            let q = Queue::from_iter(v.clone());
            v.len() == q.len()
        }

        fn order(v: Vec<i32>) -> bool {
            let q = Queue::from_iter(v.clone());
            v == Vec::from_iter(q.iter().map(|a| *a))
        }
    }
}
