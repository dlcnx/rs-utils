#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};

use encoding::all::GBK;
use encoding::{DecoderTrap, Encoding};

pub struct Subprocess {
    child: Child,
    path: Option<String>,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Subprocess {
    /// 生成子进程并启动
    pub fn new(program: &str, path: Option<String>) -> Self {
        let mut command = Command::new(program);
        command.stdin(Stdio::piped()).stdout(Stdio::piped());
        // 传入了path时进行设置
        if let Some(content) = &path {
            command.env("PATH", content);
        }
        let mut child = command.spawn().unwrap();
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        Self {
            child,
            path,
            stdin,
            stdout,
        }
    }

    /// 写入输入流一行内容并换行执行
    pub fn write_line(&mut self, cmd: &str) {
        self.stdin.write(format!("{}\n", cmd).as_bytes()).unwrap();
        self.stdin.flush().unwrap();
    }

    /// 从输出流读取一行内容并返回
    pub fn read_line(&mut self) -> String {
        let mut line: Vec<u8> = Vec::new();
        self.stdout.read_until(b'\n', &mut line).unwrap();
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
