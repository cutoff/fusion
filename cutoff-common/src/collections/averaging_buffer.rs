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
                self.sum = self.sum.saturating_sub(old);
            }
        }
        self.buffer.push_back(value);
        self.sum = self.sum.saturating_add(value);
    }

    pub fn avg(&self) -> Option<f64> {
        if self.buffer.is_empty() {
            None
        } else {
            Some(self.sum as f64 / self.buffer.len() as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buffer = AveragingBuffer::new(5);
        assert_eq!(buffer.capacity, 5);
        assert!(buffer.buffer.is_empty());
        assert_eq!(buffer.sum, 0);
    }

    #[test]
    fn test_push_and_avg() {
        let mut buffer = AveragingBuffer::new(3);

        assert_eq!(buffer.avg(), None);

        buffer.push(1);
        assert_eq!(buffer.avg(), Some(1.0));

        buffer.push(2);
        assert!((buffer.avg().unwrap() - 1.5).abs() < 1e-10);

        buffer.push(3);
        assert_eq!(buffer.avg(), Some(2.0));

        buffer.push(4);
        assert_eq!(buffer.avg(), Some(3.0));
    }

    #[test]
    fn test_overflow() {
        let mut buffer = AveragingBuffer::new(3);
        buffer.push(usize::MAX);
        buffer.push(usize::MAX);
        buffer.push(usize::MAX);
        assert_eq!(buffer.sum, usize::MAX);
        buffer.push(1);
        assert_eq!(buffer.sum, 1);
        assert!((buffer.avg().unwrap() - 0.3333333333333333).abs() < 1e-10);
    }
}