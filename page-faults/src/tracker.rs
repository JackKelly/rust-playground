use std::collections::VecDeque;

pub(crate) struct TrackerUsingBox<T> {
    pub(crate) ops_in_flight: Vec<Option<Box<T>>>,
    pub(crate) next_index: VecDeque<usize>,
}

impl<T> TrackerUsingBox<T> {
    pub(crate) fn new(n: usize) -> Self {
        Self {
            ops_in_flight: (0..n).map(|_| None).collect(),
            next_index: (0..n).collect(),
        }
    }

    pub(crate) fn get_next_index(&mut self) -> Option<usize> {
        self.next_index.pop_front()
    }

    pub(crate) fn put(&mut self, index: usize, op: T) {
        let op = Box::new(op);
        self.ops_in_flight[index].replace(op);
    }

    pub(crate) fn as_mut(&mut self, index: usize) -> Option<&mut T> {
        self.ops_in_flight[index].as_mut().map(|t| t.as_mut())
    }

    pub(crate) fn remove(&mut self, index: usize) -> Option<T> {
        self.ops_in_flight[index].take().map(|t| {
            self.next_index.push_back(index);
            *t
        })
    }
}

pub(crate) struct Tracker<T> {
    pub(crate) ops_in_flight: Vec<Option<T>>,
    pub(crate) next_index: VecDeque<usize>,
}

impl<T> Tracker<T> {
    pub(crate) fn new(n: usize) -> Self {
        Self {
            ops_in_flight: (0..n).map(|_| None).collect(),
            next_index: (0..n).collect(),
        }
    }

    pub(crate) fn get_next_index(&mut self) -> Option<usize> {
        self.next_index.pop_front()
    }

    pub(crate) fn put(&mut self, index: usize, op: T) {
        self.ops_in_flight[index].replace(op);
    }

    pub(crate) fn as_mut(&mut self, index: usize) -> Option<&mut T> {
        self.ops_in_flight[index].as_mut()
    }

    pub(crate) fn remove(&mut self, index: usize) -> Option<T> {
        self.ops_in_flight[index].take().map(|t| {
            self.next_index.push_back(index);
            t
        })
    }
}
