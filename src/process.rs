use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::io::{Read, Write};

extern crate threadpool;
use self::threadpool::ThreadPool;

pub struct Group {
    key: String,
    data: String,
}

impl Group {
    pub fn new(key: String, data: String) -> Group {
        Group {key: key, data: data}
    }
}

pub struct GroupProcResult {
    pub idx: usize,
    pub key: String,
    pub data: Option<String>,
}

pub struct Process {
    tx: mpsc::Sender<GroupProcResult>,
    rx: mpsc::Receiver<GroupProcResult>,
    cmd: String,
    pool: ThreadPool,
    counter: usize,
}

impl Process {
    pub fn new(cmd : &str, pool_size : usize) -> Process {
        let (tx, rx) = mpsc::channel();
        Process {
            tx: tx,
            rx: rx,
            cmd: cmd.to_owned(),
            pool: ThreadPool::new(pool_size),
            counter: 0,
        }
    }

    fn increment(&mut self) { self.counter += 1 }

    pub fn decrement(&mut self) { self.counter -= 1 }

    pub fn push(&mut self, grp: Group) {
        let buf = grp.data;
        let key = grp.key;
        let tx  = self.tx.clone();
        let cmd = self.cmd.clone();

        // Increment the counter so we know how many results we
        // have to fetch from the channel
        self.increment();
        let idx = self.counter;
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
            let _ = process.stdout.as_mut().unwrap().read_to_string(&mut bufout);
            let proc_result = GroupProcResult {idx: idx, key: key, data: Some(bufout)};
            tx.send(proc_result).expect("sending to channel failed");
        });
    }

    pub fn packets(&mut self) -> ProcessIntoIterator {
        ProcessIntoIterator {subprocess: self}
    }
}

pub struct ProcessIntoIterator<'a> {
     subprocess: &'a mut Process,
}

impl <'a>Iterator for ProcessIntoIterator<'a> {
    type Item = GroupProcResult;

    fn next(&mut self) -> Option<GroupProcResult> {
        // TODO Investigate why the try_recv! version does not work.
        let counter = self.subprocess.counter;
        if counter > 0 {
            let result = self.subprocess.rx.recv().unwrap();
            self.subprocess.decrement();
            Some(result)
        }
        else {
            None
        }
    }
}
