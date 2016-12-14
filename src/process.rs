use std::sync::mpsc;
use std::process::{Command, Stdio};
use std::io::{Read, Write};

extern crate threadpool;
use self::threadpool::ThreadPool;

pub struct Process {
    tx: mpsc::Sender<Option<String>>,
    pub rx: mpsc::Receiver<Option<String>>,
    cmd: String,
    pool: ThreadPool,
}

impl Process {
    pub fn new(cmd : &str, pool_size : usize) -> Process {
        let (tx, rx) = mpsc::channel();
        Process {
            tx: tx,
            rx: rx,
            cmd: cmd.to_owned(),
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
            process.stdout.as_mut().unwrap().read_to_string(&mut bufout).ok();
            tx.send(Some(bufout)).expect("sending to channel failed");
        });
    }

}

// pub struct ProcessIntoIterator<'a> {
//      subprocess: &'a mut Process,
// }

// impl <'a>Iterator for ProcessIntoIterator<'a> {
//     type Item = String;
//     fn next(&mut self) -> Option<String> {
//         let data = self.subprocess.rx.try_recv();
//         if data.is_ok() {
//             data.unwrap()
//         }
//         else {
//             None
//         }
//     }
// }
