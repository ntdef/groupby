use std;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::io::Read;

extern crate threadpool;

use self::threadpool::ThreadPool;

pub struct Process {
    // process: std::process::Child,
    tx: mpsc::Sender<Option<String>>,
    pub rx: mpsc::Receiver<Option<String>>,
    cmd: String,
    children: Vec<JoinHandle<()>>,
    pool: ThreadPool,
}

impl Process {
    pub fn new(cmd : &str, pool_size : usize) -> Process {
        let (tx, rx) = mpsc::channel();
        Process {
            tx: tx,
            rx: rx,
            cmd: cmd.to_owned(),
            children: vec![],
            pool: ThreadPool::new(pool_size),
        }
    }

    pub fn push(&mut self, buf: String) {
        let tx  = self.tx.clone();
        let cmd = self.cmd.clone();
        self.pool.execute(move || {
            let mut process =
                Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            process.stdin.as_mut()
                .expect("unwrapping stdin did not work")
                .write_all(&buf.as_bytes()).ok();
            let _ = process.wait();
            let mut bufout = String::new();
            process.stdout.as_mut().unwrap().read_to_string(&mut bufout);
            tx.send(Some(bufout)).expect("sending to channel failed");
            // let reader = BufReader::new(process.stdout.as_mut().unwrap());
            // for line in reader.lines() {
            //     let line = line.unwrap();
            //     // println!("{}", line.clone());
            //     tx.send(Some(line)).expect("sending to channel failed");
            // }
        });
    }

}

pub struct ProcessIntoIterator<'a> {
     subprocess: &'a mut Process,
}

impl <'a>Iterator for ProcessIntoIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let data = self.subprocess.rx.try_recv();
        if data.is_ok() {
            data.unwrap()
        }
        else {
            None
        }
    }
}
