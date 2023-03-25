#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, Command, ExitStatus, Stdio};

use encoding::all::GBK;
use encoding::{DecoderTrap, Encoding};

pub struct Subprocess {
    child: Child,
    path: Option<String>,
}

impl Subprocess {
    /// 生成子进程并启动
    pub fn new(program: &str, path: Option<String>) -> Self {
        let mut p = Command::new(program);
        p.stdin(Stdio::piped()).stdout(Stdio::piped());
        // 传入了path时进行设置
        if let Some(content) = &path {
            p.env("PATH", content);
        }
        let p = p.spawn().unwrap();
        Self { child: p, path }
    }

    /// 写入输入流一行内容并换行执行
    pub fn write_line(&mut self, cmd: String) {
        let p_stdin = self.child.stdin.as_mut().unwrap();
        p_stdin.write(format!("{}\n", cmd).as_bytes()).unwrap();
        p_stdin.flush().unwrap();
    }

    /// 从输出流读取一行内容并返回
    pub fn read_line(&mut self) -> String {
        let mut p_stdout = BufReader::new(self.child.stdout.as_mut().unwrap());
        let mut line: Vec<u8> = Vec::new();
        p_stdout.read_until(b'\n', &mut line).unwrap();
        // 判断utf-8或gbk编码
        if let Ok(line) = String::from_utf8(line.clone()) {
            return format!("{}", line);
        }
        format!("{}", GBK.decode(&line, DecoderTrap::Strict).unwrap())
    }

    /// 等待程序终止并返回退出码
    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        self.child.wait()
    }

    /// 获取程序运行pid
    pub fn id(&self) -> u32 {
        self.child.id()
    }

    /// 强行终止进程
    pub fn kill(&mut self) -> io::Result<()> {
        self.child.kill()
    }
}

#[ignore]
#[test]
fn test_bds() {
    let mut p = Subprocess::new(
        "bedrock_server",
        Some("D:\\ABCDEFG\\Objects\\bds\\bedrock-server-1.19.72.01".to_string()),
    );
    p.write_line("list".to_string());
    // let temp = Arc::new(Mutex::new(p));
    // let read = Arc::clone(&temp);
    // let write = Arc::clone(&temp);

    // let t = std::thread::spawn(move || loop {
    //     print!("{}", read.lock().unwrap().read_line());
    // });
    // std::thread::spawn(move || loop {
    //     std::thread::sleep(Duration::from_millis(10));
    //     let mut line = String::new();
    //     std::io::stdin().read_line(&mut line).unwrap();
    //     write.lock().unwrap().write_line(line);
    // });
    // t.join().unwrap();
}
