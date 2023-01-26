// A Queue implementation that fails to enqueue items with a given probability

use rand::Rng;

pub struct BogoQueue<T> {
    data: Vec<T>,
    fail_chance: u32,
}

impl<T> BogoQueue<T> {
    pub fn new(fail_chance: u32) -> Self {
        BogoQueue {
            data: Vec::new(),
            fail_chance,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        let rand = rand::thread_rng().gen_range(0..self.fail_chance);

        if rand != 0 {
            self.data.push(item);
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    pub fn set_fail_chance(&mut self, fail_chance: u32) {
        self.fail_chance = fail_chance;
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bogoqueue() {
        let mut queue = BogoQueue::new(10000);

        for i in 0..500 {
            queue.enqueue(i);
        }

        for i in 0..500 {
            assert_eq!(queue.dequeue(), Some(i));
        }

        assert_eq!(queue.dequeue(), None);
    }
}
