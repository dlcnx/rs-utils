#[cfg(test)]
mod test {
    use crate::LifoQueue;
    use crate::ThreadPool;

    #[test]
    fn lifo_queue() {
        let mut q = LifoQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(false, q.empty());
        assert_eq!(2, q.len());
        assert_eq!(2, q.pop().unwrap());
        assert_eq!(1, q.pop().unwrap());
        assert_eq!(true, q.empty());
    }

    #[test]
    fn thread_pool() {
        let pool = ThreadPool::new(2);
        pool.execute(|| println!("1"));
        pool.execute(|| println!("2"));
        pool.execute(|| println!("3"));
        pool.execute(|| println!("4"));
        pool.execute(|| println!("5"));
    }
}
