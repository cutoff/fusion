//! Pre-allocated, &self only, non-locking Vector

use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
struct State<T> {
    items: Vec<T>,
    /// Index of the next last item (0 for empty)
    back_index: usize,
}

pub struct PreallocatedVec<T, C> {
    state: UnsafeCell<State<T>>,
    creation_fn: C,
}

impl<T, C> Debug for PreallocatedVec<T, C>
where
    C: Fn() -> T,
    T: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.raw_slice())
    }
}

unsafe impl<T, C> Send for PreallocatedVec<T, C> {}
unsafe impl<T, C> Sync for PreallocatedVec<T, C> {} // TODO really?

impl<T, C> PreallocatedVec<T, C>
where
    C: Fn() -> T,
{
    pub fn with_capacity(capacity: usize, creation_fn: C) -> Self {
        let instance = Self {
            state: UnsafeCell::new(State {
                items: Vec::with_capacity(capacity),
                back_index: 0,
            }),
            creation_fn,
        };
        Self::set_capacity(&instance, capacity);
        instance
    }

    pub fn capacity(&self) -> usize {
        unsafe {
            let state = &mut* self.state.get();
            state.items.capacity()
        }
    }

    pub fn set_capacity(&self, capacity: usize) {
        unsafe {
            let state = &mut* self.state.get();
            state.items.resize_with(capacity, &self.creation_fn);
            state.back_index = state.back_index.min(capacity);
        }
    }

    pub fn raw_slice(&self) -> &[T] {
        unsafe {
            let state = &mut* self.state.get();
            state.items.as_slice()
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            let state = &mut *self.state.get();
            state.back_index
        }
    }

    pub fn clear(&self) {
        unsafe {
            let state = &mut* self.state.get();
            state.back_index = 0;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        unsafe {
            let state = &mut *self.state.get();
            if index < state.back_index {
                state.items.get(index)
            } else {
                None
            }
        }
    }

    pub fn get_mut(&self, index: usize) -> Option<&mut T> {
        unsafe {
            let state = &mut *self.state.get();
            if index < state.back_index {
                state.items.get_mut(index)
            } else {
                None
            }
        }
    }

    pub fn last(&self) -> Option<&T> {
        unsafe {
            let state = &mut *self.state.get();
            let len = state.items.len();
            if len == 0 {
                None
            } else {
                state.items.get(len - 1)
            }
        }
    }

    pub fn last_mut(&self) -> Option<&mut T> {
        unsafe {
            let state = &mut *self.state.get();
            let len = state.items.len();
            if len == 0 {
                None
            } else {
                state.items.get_mut(len - 1)
            }
        }
    }
    
    pub fn last_index(&self) -> Option<usize> {
        let len = self.len();
        if len == 0 {
            None
        } else {
            Some(len - 1)
        }
    }

    pub fn push(&self, new_item: T) {
        unsafe {
            let state = &mut *self.state.get();
            if let Some(item) = state.items.get_mut(state.back_index) {
                *item = new_item;
            } else {
                // Capacity overflow: we push the event at the expanse of a potential new allocation
                state.items.push(new_item);
            }
            state.back_index += 1;
        }
    }

    pub fn push_prealloc<F>(&self, copy_fn: F)
    where
        F: Fn(&mut T) -> bool, // TODO Replace bool by Result
    {
        unsafe {
            let state = &mut *self.state.get();
            if let Some(item) = state.items.get_mut(state.back_index) {
                if !copy_fn(item) {
                    // abort
                    return;
                }
            } else {
                // Capacity overflow: we push the event at the expanse of a potential new allocation
                let mut item = (self.creation_fn)();
                if !copy_fn(&mut item) {
                    // abort
                    return;
                }
                state.items.push(item);
            }
            state.back_index += 1;
        }
    }
    
    // TODO iter, enumerate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let v = PreallocatedVec::with_capacity(10, || 0usize);

        v.push(0usize);
        v.push(1usize);
        v.push(2usize);
        println!("Vector: {:?}. Len: {}. Capacity: {}", v, v.len(), v.capacity());

        v.clear();
        println!("Vector: {:?}. Len: {}. Capacity: {}", v, v.len(), v.capacity());

        v.push(3usize);
        v.push(4usize);
        println!("Vector: {:?}. Len: {}. Capacity: {}", v, v.len(), v.capacity());
    }
}