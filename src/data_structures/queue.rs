type Index = usize;

///
/// ```
/// use primeagen_algo_course::data_structures::queue::Queue;
///
/// let mut queue = Queue::<i32>::new(4);
/// queue.enqueue(5);
/// queue.enqueue(6);
/// queue.enqueue(2);
/// 
/// assert_eq!(queue.len(), 3);
/// 
/// assert_eq!(*queue.dequeue(), 5);
/// assert_eq!(*queue.dequeue(), 6);
/// assert_eq!(*queue.dequeue(), 2);
/// ```
///
pub struct Queue<T> {
    head: Index,
    tail: Index,
    data: Vec<T>,
}


impl<T> Queue<T> where T: Clone {
    pub fn new(size: usize) -> Self {
        Queue {
            head: 0,
            tail: 0,
            data: Vec::with_capacity(size),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn enqueue(&mut self, value: T) {
        // TODO: wrap around
        self.data.push(value);
        self.tail += 1;
    }

    pub fn dequeue(&mut self) -> &T {
        // TODO: wrap around
        let value = &self.data[self.head];
        self.head += 1;
        value
    }
}
