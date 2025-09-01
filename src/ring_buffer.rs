use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    size: usize,
    capacity: usize,
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            head: 0,
            size: 0,
            capacity,
        }
    }

    pub fn push(&mut self, item: T) {
        self.buffer[self.head] = Some(item);
        self.head = (self.head + 1) % self.capacity;
        if self.size < self.capacity {
            self.size += 1;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        let start = if self.size == self.capacity {
            self.head
        } else {
            0
        };

        (0..self.size).map(move |i| {
            let idx = (start + i) % self.capacity;
            self.buffer[idx].as_ref().unwrap()
        })
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}