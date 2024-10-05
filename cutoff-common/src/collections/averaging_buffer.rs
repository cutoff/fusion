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

    pub fn avg(&self) -> Option<usize> {
        if self.buffer.is_empty() {
            None
        } else {
            Some(self.sum / self.buffer.len())
        }
    }

    pub fn std_deviation(&self) -> Option<f64> {
        if self.buffer.len() < 2 {
            return None;
        }

        let mean = self.avg().unwrap() as f64;
        let variance = self.buffer.iter()
            .map(|&x| {
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum::<f64>() / self.buffer.len() as f64;

        Some(variance.sqrt())
    }

    pub fn sample_deviation(&self) -> Option<f64> {
        if self.buffer.len() < 2 {
            return None;
        }

        let mean = self.avg().unwrap() as f64;
        let variance = self.buffer.iter()
            .map(|&x| {
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum::<f64>() / (self.buffer.len() - 1) as f64;

        Some(variance.sqrt())
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
        assert_eq!(buffer.avg(), Some(1));

        buffer.push(2);
        assert_eq!(buffer.avg(), Some(1));

        buffer.push(3);
        assert_eq!(buffer.avg(), Some(2));

        buffer.push(4);
        assert_eq!(buffer.avg(), Some(3));
    }

    #[test]
    fn test_overflow() {
        let mut buffer = AveragingBuffer::new(3);
        buffer.push(usize::MAX);
        buffer.push(usize::MAX);
        buffer.push(usize::MAX);
        assert_eq!(buffer.sum, usize::MAX);
        buffer.push(1);
        assert_eq!(buffer.sum, usize::MAX);
    }

    #[test]
    fn test_std_deviation() {
        let mut buffer = AveragingBuffer::new(5);
        assert_eq!(buffer.std_deviation(), None);

        buffer.push(2);
        assert_eq!(buffer.std_deviation(), None);

        buffer.push(4);
        buffer.push(4);
        buffer.push(4);
        buffer.push(6);

        let std_dev = buffer.std_deviation().unwrap();
        assert!((std_dev - std::f64::consts::SQRT_2).abs() < 1e-10);
    }

    #[test]
    fn test_sample_deviation() {
        let mut buffer = AveragingBuffer::new(5);
        assert_eq!(buffer.sample_deviation(), None);

        buffer.push(2);
        assert_eq!(buffer.sample_deviation(), None);

        buffer.push(4);
        buffer.push(4);
        buffer.push(4);
        buffer.push(6);

        let sample_dev = buffer.sample_deviation().unwrap();
        assert!((sample_dev - 1.5811388300841898).abs() < 1e-10);
    }
}