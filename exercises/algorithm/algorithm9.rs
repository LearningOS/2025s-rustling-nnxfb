/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::{Debug, Display};

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;

        let mut current = self.count;
        while current != 1 {
            let parent = current / 2;
            if (self.comparator)(&self.items[current], &self.items[parent]) {
                self.items.swap(current, parent);
            } else {
                break;
            }
            current = parent;
        }
    }

    // fn parent_idx(&self, idx: usize) -> usize {
    //     idx / 2
    // }

    // fn children_present(&self, idx: usize) -> bool {
    //     self.left_child_idx(idx) <= self.count
    // }

    // fn left_child_idx(&self, idx: usize) -> usize {
    //     idx * 2
    // }

    // fn right_child_idx(&self, idx: usize) -> usize {
    //     self.left_child_idx(idx) + 1
    // }

    // fn smallest_child_idx(&self, idx: usize) -> usize {
    //     //TODO
	// 	0
    // }
}

impl<T> Heap<T>
where
    T: Default + Ord + Copy + Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let ret;
        if self.count != 0 {
            ret = self.items[1];
            self.items[1] = self.items.pop().unwrap();
            self.count -= 1;
        } else {
            return None;
        }
        
        let mut current = 1;
        loop {
            let lchild = current * 2;
            let rchild = current * 2 + 1;

            if rchild <= self.count {
                if (self.comparator)(&self.items[lchild], &self.items[rchild]) {
                    self.items.swap(current, lchild);
                    current = lchild;
                } else {
                    self.items.swap(current, rchild);
                    current = rchild;
                }
            } else if lchild < self.count {
                self.items.swap(current, lchild);
                current = lchild;
            } else {
                break;
            }
        };
        Some(ret)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}