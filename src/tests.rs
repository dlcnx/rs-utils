use crate::ThreadPool;
use crate::Subprocess;

#[test]
fn thread_pool() {
    let pool = ThreadPool::new(2);
    pool.execute(|| println!("1"));
    pool.execute(|| println!("2"));
    pool.execute(|| println!("3"));
    pool.execute(|| println!("4"));
    pool.execute(|| println!("5"));
}

#[test]
fn subprocess() {
    let mut p = Subprocess::new("cmd", None);
    p.write_line("echo 123".to_string());
    p.read_line();
    p.wait().unwrap();
}
