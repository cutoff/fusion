//! A buffer that maintains a running average of its elements.
//!
//! This module provides the `AveragingBuffer` struct, which is a fixed-capacity
//! buffer that maintains a running average of the values it contains.

use std::collections::VecDeque;

/// A fixed-capacity buffer that maintains a running average of its elements.
///
/// The `AveragingBuffer` stores a fixed number of `usize` values and provides
/// methods to add new values and calculate the average of all values currently
/// in the buffer. When the buffer reaches its capacity, adding a new value will
/// remove the oldest value.
///
/// The buffer uses saturating arithmetic to prevent overflow when calculating the sum.
///
/// # Examples
///
/// ```
/// use cutoff_common::collections::averaging_buffer::AveragingBuffer;
///
/// // Create a buffer with capacity 3
/// let mut buffer = AveragingBuffer::new(3);
///
/// // Add some values
/// buffer.push(1);
/// buffer.push(2);
/// buffer.push(3);
///
/// // Calculate the average
/// assert_eq!(buffer.avg(), Some(2.0));
///
/// // Add another value, which will push out the oldest value (1)
/// buffer.push(4);
/// assert_eq!(buffer.avg(), Some(3.0)); // Average of [2, 3, 4]
/// ```
#[derive(Debug, Clone, Default)]
pub struct AveragingBuffer {
    /// The internal buffer storing the values
    buffer: VecDeque<usize>,
    /// The maximum number of elements the buffer can hold
    capacity: usize,
    /// The sum of all elements in the buffer, used for efficient average calculation
    sum: usize,
}

impl AveragingBuffer {
    /// Creates a new `AveragingBuffer` with the specified capacity.
    ///
    /// # Parameters
    ///
    /// * `capacity` - The maximum number of elements the buffer can hold.
    ///
    /// # Returns
    ///
    /// A new, empty `AveragingBuffer` with the specified capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use cutoff_common::collections::averaging_buffer::AveragingBuffer;
    ///
    /// let buffer = AveragingBuffer::new(5);
    /// assert_eq!(buffer.avg(), None); // Empty buffer has no average
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            sum: 0,
        }
    }

    /// Adds a value to the buffer.
    ///
    /// If the buffer is at capacity, the oldest value will be removed.
    /// The sum is updated using saturating arithmetic to prevent overflow.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to add to the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use cutoff_common::collections::averaging_buffer::AveragingBuffer;
    ///
    /// let mut buffer = AveragingBuffer::new(2);
    /// buffer.push(1);
    /// buffer.push(2);
    /// assert_eq!(buffer.avg(), Some(1.5));
    ///
    /// // Adding a third value will remove the first value
    /// buffer.push(3);
    /// assert_eq!(buffer.avg(), Some(2.5)); // Average of [2, 3]
    /// ```
    pub fn push(&mut self, value: usize) {
        if self.buffer.len() == self.capacity {
            if let Some(old) = self.buffer.pop_front() {
                // Use saturating subtraction to prevent underflow
                self.sum = self.sum.saturating_sub(old);
            }
        }
        self.buffer.push_back(value);
        // Use saturating addition to prevent overflow
        self.sum = self.sum.saturating_add(value);
    }

    /// Calculates the average of all values in the buffer.
    ///
    /// # Returns
    ///
    /// * `Some(f64)` - The average of all values in the buffer.
    /// * `None` - If the buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use cutoff_common::collections::averaging_buffer::AveragingBuffer;
    ///
    /// let mut buffer = AveragingBuffer::new(3);
    /// assert_eq!(buffer.avg(), None); // Empty buffer
    ///
    /// buffer.push(1);
    /// buffer.push(2);
    /// buffer.push(3);
    /// assert_eq!(buffer.avg(), Some(2.0)); // Average of [1, 2, 3]
    /// ```
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
