use std::cmp::Ordering;

struct PriorityQueue<T> {
    heap: Vec<T>,
}

impl<T: Ord> PriorityQueue<T> {
    fn new() -> PriorityQueue<T> {
        PriorityQueue { heap: Vec::new() }
    }

    fn push(&mut self, elem: T) {
        self.heap.push(elem);
        let len = self.heap.len();
        self.sift_up(len - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let len = self.heap.len();
        let last_elem = self.heap.swap_remove(len - 1);
        if len > 1 {
            let first_elem = std::mem::replace(&mut self.heap[0], last_elem);
            self.sift_down(0, first_elem);
        }
        Some(last_elem)
    }

    fn sift_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }
        let parent_idx = (idx - 1) / 2;
        if self.heap[idx] < self.heap[parent_idx] {
            self.heap.swap(idx, parent_idx);
            self.sift_up(parent_idx);
        }
    }

    fn sift_down(&mut self, idx: usize, elem: T) {
        let left_child_idx = idx * 2 + 1;
        let right_child_idx = idx * 2 + 2;
        let mut min_child_idx = idx;
        if left_child_idx < self.heap.len()
            && Ordering::Less == self.heap[left_child_idx].cmp(&self.heap[min_child_idx])
        {
            min_child_idx = left_child_idx;
        }
        if right_child_idx < self.heap.len()
            && Ordering::Less == self.heap[right_child_idx].cmp(&self.heap[min_child_idx])
        {
            min_child_idx = right_child_idx;
        }
        if min_child_idx != idx {
            self.heap.swap(idx, min_child_idx);
            self.sift_down(min_child_idx, elem);
        } else {
            self.heap[idx] = elem;
        }
    }
}

fn main() {
    let mut pq = PriorityQueue::new();
    pq.push(3);
    pq.push(1);
    pq.push(2);
    while let Some(x) = pq.pop() {
        println!("{}", x);
    }
}
