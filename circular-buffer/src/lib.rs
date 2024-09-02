use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buffer.len() == self.capacity {
            Err(Error::FullBuffer)
        } else {
            self.buffer.push_back(element);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            Ok(self.buffer.pop_front().unwrap())
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(element);
    }
}
