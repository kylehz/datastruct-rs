pub struct VecDeque<T> {
    vec: Vec<Option<T>>,
    head: usize,
    tail: usize,
}

const INITIAL_CAPACITY: usize = 3;

impl<T> VecDeque<T>
where
    T: Clone,
{
    pub fn new() -> VecDeque<T> {
        let vec = Vec::new();
        let mut vecq = VecDeque {
            head: 0,
            tail: 0,
            vec: vec,
        };
        vecq.grow(INITIAL_CAPACITY);
        vecq
    }
    fn grow(&mut self, additional: usize) {
        let old_len = self.vec.len();
        self.vec.reserve(additional);
        for i in 0..additional {
            self.vec.push(None);
        }
        if self.head > self.tail {
            for i in (self.head..old_len).rev() {
                self.vec[i + additional] = self.vec[i].take();
            }
            self.head = self.head + additional
        }
    }
    fn grow_if_necessary(&mut self) {
        if self.len() + 1 >= self.cap() {
            self.grow(self.cap());
        }
    }
    pub fn get_vec(&self) -> Vec<Option<T>> {
        self.vec.clone()
    }
    pub fn is_empty(&self) -> bool {
        self.tail == self.head
    }
    pub fn is_full(&self) -> bool {
        self.head - self.tail == 1
    }
    pub fn len(&self) -> usize {
        if self.head <= self.tail {
            self.tail - self.head
        } else {
            self.cap() - (self.head - self.tail)
        }
    }
    pub fn cap(&self) -> usize {
        self.vec.len()
    }
    pub fn push_back(&mut self, val: T) {
        self.grow_if_necessary();
        self.vec[self.tail] = Some(val);
        self.tail = (self.tail + 1) % self.cap();
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail == self.head {
            None
        } else {
            self.tail = (self.tail + self.cap() - 1) % self.cap();
            self.vec[self.tail].take()
        }
    }
    pub fn push_front(&mut self, val: T) {
        self.grow_if_necessary();
        self.head = (self.head + self.cap() - 1) % self.cap();
        self.vec[self.head] = Some(val);
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.tail == self.head {
            None
        } else {
            let val = self.vec[self.head].take();
            self.head = (self.head + 1) % self.cap();
            val
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut vecq = VecDeque::<i32>::new();
        vecq.push_back(1);
        vecq.push_back(2);
        vecq.push_front(3);
        vecq.push_front(4);
        vecq.push_back(5);
        vecq.pop_back();
        vecq.pop_front();

        vecq.push_back(6);

        assert_eq!(
            vecq.get_vec(),
            vec![Some(1), Some(2), Some(6), None, None, Some(3)]
        );
    }
}
