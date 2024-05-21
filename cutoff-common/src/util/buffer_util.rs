use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct AveragingBuffer {
    buffer: VecDeque<usize>,
    capacity: usize,
    sum: usize,
}

impl AveragingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            sum: 0,
        }
    }

    pub fn push(&mut self, value: usize) {
        if self.buffer.len() == self.capacity {
            if let Some(old) = self.buffer.pop_front() {
                self.sum -= old;
            }
        }
        self.buffer.push_back(value);
        self.sum += value;
    }

    pub fn avg(&self) -> Option<usize> {
        if self.buffer.is_empty() {
            None
        } else {
            Some(self.sum / self.buffer.len())
        }
    }

    pub fn std_deviation(&self) -> Option<u128> {
        todo!()
    }

    pub fn sample_deviation(&self) -> Option<u128> {
        todo!()
    }
}
