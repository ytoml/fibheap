use std::collections::HashMap;
use std::mem;

#[derive(Debug)]
struct Node<T> {
    value: T,
    children: Vec<Self>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, children: vec![] }
    }

    fn value(&self) -> &T {
        &self.value
    }

    fn children(&self) -> &[Self] {
        &self.children
    }

    fn degree(&self) -> usize {
        self.children.len()
    }

    fn push_child(&mut self, node: Self) {
        self.children.push(node);
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            children: self.children.clone()
        }
    }
}

#[derive(Debug)]
pub struct FibonacciHeap<T> {
   roots: Vec<Node<T>>,
   top_index: usize,
   len: usize, // count of whole nodes (not self.roots.len())
}

impl<T: PartialOrd> FibonacciHeap<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        if vec.is_empty() {
            return Self::new();
        }

        let len = vec.len();
        let mut roots = Vec::with_capacity(len);
        let top_index = len-1; // minimum value will be inserted at last
        let mut min_cell = None;
        for mut value in vec.into_iter() {
            if let Some(mut min_val) = min_cell.take() {
                if value > min_val {
                    mem::swap(&mut value, &mut min_val)
                }
                roots.push(Node::new(min_val));
            } 
            let _ = min_cell.insert(value);
        }
        if let Some(min_val) = min_cell {
            roots.push(Node::new(min_val));
        }
        Self { roots, top_index, len }
    }

    pub fn into_vec(mut self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len());
        while let Some(value) = self.pop() {
            vec.push(value);
        }
        vec
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn top(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(self.roots[self.top_index].value())
        }
    }

    pub fn push(&mut self, value: T) {
        if !self.roots.is_empty() {
            let cur = self.roots[self.top_index].value();
            if &value < cur {
                self.top_index = self.roots.len();
            }
        }
        self.roots.push(Node::new(value));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // degree -> (new) root
        // Using Rc<RefCell> here is actually
        // just to use .and_modify().or_insert() when updating, avoiding use after move.
        let max_new_roots_capacity = self.roots.len() - 1 + self.roots[self.top_index].children().len();
        let mut deg_to_root: HashMap<usize, Node<T>> =  HashMap::with_capacity(max_new_roots_capacity);

        let roots = mem::take(&mut self.roots);
        let mut ret = None;
        for (ix , node) in roots.into_iter().enumerate() {
            if ix == self.top_index {
                let Node { value, children } = node;
                for node in children.into_iter() {
                    map_update(&mut deg_to_root, node);
                }
                let _ = ret.insert(value);
            } else {
                map_update(&mut deg_to_root, node);
            }
        }

        if !deg_to_root.is_empty() {
            let len = deg_to_root.len();
            self.roots.reserve(len);
            self.top_index = len - 1; // minimum value will be inserted at last
            self.len -= 1;

            let mut min_cell: Option<Node<T>> = None;
            for (_, mut node) in deg_to_root.into_iter() {
                if let Some(mut other) = min_cell.take() {
                    if node.value() > other.value() {
                        mem::swap(&mut node, &mut other);
                    }
                    self.roots.push(other);
                }
                let _ = min_cell.insert(node);
            }
            if let Some(node) = min_cell {
                self.roots.push(node);
            }
        } else {
            self.top_index = 0;
            self.len = 0;
        }
        ret
    }

    pub fn append(&mut self, other: FibonacciHeap<T>) {
        if other.is_empty() {
            return;
        }
        if self.is_empty() {
            self.roots = other.roots;
            self.top_index = other.top_index;
            self.len = other.len;
            return;
        }
        let FibonacciHeap { mut roots, len, top_index } = other;
        if self.top().unwrap() > roots[top_index].value() {
            self.top_index = self.roots.len() + top_index;
        }
        self.roots.append(&mut roots);
        self.len += len;
    }
}

fn map_update<T: PartialOrd>(deg_to_root: &mut HashMap<usize, Node<T>>, mut node: Node<T>) {
    let deg = node.degree();
    if let Some(mut root) = deg_to_root.remove(&deg) {
        // Root must be with smaller value
        if node.value() < root.value() {
            mem::swap(&mut node, &mut root);
        }
        root.push_child(node);
        map_update(deg_to_root, root);
    } else {
        deg_to_root.insert(deg, node);
    }
}

impl<T: PartialOrd> Iterator for FibonacciHeap<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<T: PartialOrd> Default for FibonacciHeap<T> {
    fn default() -> Self {
        Self { roots: vec![], top_index: 0, len: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        let heap = FibonacciHeap::<i32>::new();
        assert!(heap.top().is_none());

        let heap = FibonacciHeap::<i32>::from_vec(vec![]);
        assert!(heap.top().is_none());
    }

    #[test]
    fn pop() {
        let mut heap = FibonacciHeap::<i32>::from_vec(vec![3, 5, 1, 9]);
        assert_eq!(heap.pop().unwrap(), 1);
        assert_eq!(heap.pop().unwrap(), 3);
        assert_eq!(heap.pop().unwrap(), 5);
        assert_eq!(heap.pop().unwrap(), 9);
    }

    #[test]
    fn append() {
        let mut heap = FibonacciHeap::<i32>::from_vec(vec![3, 5, 1, 9]);
        let heap2 = FibonacciHeap::<i32>::from_vec(vec![8, 2, 7, 4, 6]);
        heap.append(heap2);
        for i in 1..=9 {
            // TODO: This produces 1, 2, 3, 4, 6, 5, 7, 8, 9 (5, 6 are not correct order);
            assert_eq!(heap.pop().unwrap(), i);
        }
    }

    #[test]
    fn pop_large() {
        let mut heap = FibonacciHeap::new();
        for i in (0..1000000).rev() {
            heap.push(i);
        }
        for (i, v) in heap.into_iter().enumerate() {
            assert_eq!(i, v);
        }
    }
}