use rand::Rng;

pub struct BogoQueue<T> {
    data: Vec<T>,
}

impl<T> BogoQueue<T> {
    pub fn new() -> Self {
        BogoQueue { data: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        let rand = rand::thread_rng().gen_range(0..10000);

        // 1/10000 chance of not adding the item
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bogoqueue() {
        let mut queue = BogoQueue::new();

        for i in 0..500 {
            queue.enqueue(i);
        }

        for i in 0..500 {
            assert_eq!(queue.dequeue(), Some(i));
        }

        assert_eq!(queue.dequeue(), None);
    }
}
