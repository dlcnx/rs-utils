use crate::Subprocess;
use crate::ThreadPool;

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
    p.write_line("echo 123");
    p.write_line("exit");
    loop {
        let next_line = p.read_line();
        if next_line.is_empty() {
            break;
        }
        // print!("{}", next_line);
    }
    p.wait().unwrap();
}

#[ignore]
#[test]
fn test_bds() {
    let mut p = Subprocess::new(
        "bedrock_server_mod",
        Some("D:\\ABCDEFG\\Objects\\bds\\bedrock-server-1.19.72.01".to_string()),
    );
    p.write_line("list");
    p.write_line("stop");
    loop {
        let next_line = p.read_line();
        if next_line.is_empty() {
            break;
        }
        print!("{}", next_line);
    }
    p.wait().unwrap();
}
