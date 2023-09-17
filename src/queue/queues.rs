#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueError {
    CapacityOverflow,  // attempted to add to a max capacity queue
    CapacityUnderflow, // attempted to remove from an empty queue
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeekError {
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapacityType<T>
where
    T: std::ops::Add,
{
    Fixed(T),
    Dynamic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queue<T> {
    pub data: Vec<T>,
    pub capacity: CapacityType<usize>,
}

pub fn new<T>() -> Queue<T> {
    Queue {
        data: Vec::new(),
        capacity: CapacityType::Dynamic,
    }
}

pub fn from<T>(data: Vec<T>) -> Queue<T> {
    Queue {
        data,
        capacity: CapacityType::Dynamic,
    }
}

pub fn with_capacity<T>(capacity: usize) -> Queue<T> {
    Queue {
        data: Vec::with_capacity(capacity),
        capacity: CapacityType::Fixed(capacity),
    }
}

pub fn from_capacity<T>(data: Vec<T>) -> Queue<T>
where
    T: Clone,
{
    Queue {
        data: data.clone(),
        capacity: CapacityType::Fixed(data.len()),
    }
}

pub trait TQueue<T> {
    fn push(&mut self, item: T) -> Result<(), QueueError>;
    fn pop(&mut self) -> Result<(), QueueError>;
    fn flush(&mut self) -> Result<(), QueueError>;
    fn peek(&self) -> Option<&T>;
    fn peek_mut(&mut self) -> Option<&mut T>;
    fn peek_range(&self, start: usize, end: usize) -> Result<&[T], PeekError>;
    fn peek_range_mut(&mut self, start: usize, end: usize) -> Result<&mut [T], PeekError>;
    fn peek_from(&self, start: usize) -> Result<&[T], PeekError>;
    fn peek_from_mut(&mut self, start: usize) -> Result<&mut [T], PeekError>;
    fn peek_to(&self, end: usize) -> Result<&[T], PeekError>;
    fn peek_to_mut(&mut self, end: usize) -> Result<&mut [T], PeekError>;
}

impl<T> TQueue<T> for Queue<T> {
    fn push(&mut self, item: T) -> Result<(), QueueError> {
        match self.capacity {
            CapacityType::Dynamic => {
                self.data.push(item);
                Ok(())
            }

            CapacityType::Fixed(capacity) => {
                if self.data.len() == capacity {
                    Err(QueueError::CapacityOverflow)
                } else {
                    self.data.push(item);
                    Ok(())
                }
            }
        }
    }

    fn pop(&mut self) -> Result<(), QueueError> {
        if self.data.len() == 0 {
            Err(QueueError::CapacityUnderflow)
        } else {
            self.data.remove(0);
            Ok(())
        }
    }

    fn flush(&mut self) -> Result<(), QueueError> {
        self.data.clear();
        Ok(())
    }

    fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.first_mut()
    }

    fn peek_range(&self, start: usize, end: usize) -> Result<&[T], PeekError> {
        if self.data.len() < *[start, end].iter().max().unwrap() {
            Err(PeekError::OutOfBounds)
        } else {
            Ok(&self.data[start..end])
        }
    }

    fn peek_range_mut(&mut self, start: usize, end: usize) -> Result<&mut [T], PeekError> {
        if self.data.len() < *[start, end].iter().max().unwrap() {
            Err(PeekError::OutOfBounds)
        } else {
            Ok(&mut self.data[start..end])
        }
    }

    fn peek_from(&self, start: usize) -> Result<&[T], PeekError> {
        if self.data.len() < start {
            Err(PeekError::OutOfBounds)
        } else {
            Ok(&self.data[start..])
        }
    }

    fn peek_from_mut(&mut self, start: usize) -> Result<&mut [T], PeekError> {
        if self.data.len() < start {
            Err(PeekError::OutOfBounds)
        } else {
            Ok(&mut self.data[start..])
        }
    }

    fn peek_to(&self, end: usize) -> Result<&[T], PeekError> {
        if self.data.len() < end {
            Err(PeekError::OutOfBounds)
        } else {
            Ok(&self.data[..end])
        }
    }

    fn peek_to_mut(&mut self, end: usize) -> Result<&mut [T], PeekError> {
        if self.data.len() < end {
            Err(PeekError::OutOfBounds)
        } else {
            Ok(&mut self.data[..end])
        }
    }
}
