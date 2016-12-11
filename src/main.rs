use std::io;
use std::env;
use std::fs;
use std::io::{BufReader,BufRead,Write,Read};
use std::process::{Command, Stdio};

fn nth_field(s : &str, separator : &str, n : usize) -> String {
    s.split(separator).nth(n).unwrap().to_owned()
}

fn exec_with_buffer(cmd : &str, buf : &mut String) {
    let mut bufout = String::new();
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    // Write to stdin
    cmd.stdin.unwrap().write_all(buf.as_bytes()).ok();
    // Read from stdout, writing to buffer
    cmd.stdout.unwrap().read_to_string(&mut bufout).ok();
    print!("{}", bufout);
}

fn main() {
    let input = env::args().nth(1).unwrap_or(String::from("-"));
    let keycol = 4;
    let separator = ",";
    let cmd = "bash -c 'cat -n '";

    let rdr: Box<io::BufRead> = match input.as_ref() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _   => Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    let mut prev = String::new();
    let mut buf  = String::new();
    let mut itr  = rdr.lines();

    loop {
        let el = itr.next();
        match el {
            Some(el) => {
                let line = el.ok().unwrap();
                let cur = nth_field(&line, separator, keycol);
                if cur == prev || prev.is_empty() {
                    buf.push_str(&line); buf.push('\n');
                } else {
                    exec_with_buffer(cmd, &mut buf);
                    buf.clear();
                    buf.push_str(&line); buf.push('\n');
                }
                prev = cur
            },
            None => {
                exec_with_buffer(cmd, &mut buf);
                // flush_buffer(&mut buf);
                buf.clear();
                break
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        // TODO: Fill this out
    }

}
