/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DONE

use std::cmp::Ord;
use std::default::Default;

#[derive(Debug)]
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
        println!("***");
        self.items.push(value);
        self.count += 1;
        self.heapify_up();
    }
    fn heapify_up(&mut self) {
    	let mut start_idx = self.parent_idx(self.count);
    	while start_idx > 0 {
            let idx = self.smallest_child_idx(start_idx);
            let parent_val = &self.items[start_idx];
            let child_val = &self.items[idx];
            if (self.comparator)(child_val,parent_val) {
                self.items.swap(start_idx, idx);
            }
            
    		start_idx -= 1;
    	}
    }

    fn heapify_down(&mut self) {
        let mut start_idx = 1;
        while self.children_present(start_idx) {
        	let idx = self.smallest_child_idx(start_idx);
            let parent_val = &self.items[start_idx];
            let child_val = &self.items[idx];
            if (self.comparator)(child_val,parent_val) {
                self.items.swap(start_idx, idx);
            }
            start_idx += 1;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) <= self.count {
            let left_val = &self.items[self.left_child_idx(idx)];
            let right_val = &self.items[self.right_child_idx(idx)];
            if (self.comparator)(left_val, right_val) {
                return self.left_child_idx(idx);
            } else {
                return self.right_child_idx(idx);
            }
        }

        self.left_child_idx(idx)
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default,
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.is_empty() {
            true => return None,
            false => {
                let val = self.items.pop().unwrap();
                self.count -= 1;
                let result = self.items[1].clone();
                self.items[1] = val;
                self.heapify_down();
                return Some(result);
            },
        }
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