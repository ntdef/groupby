use std::io;
use std::env;
use std::fs;
use std::io::{BufReader,BufRead,Write,Read};
use std::process::{Command, Stdio};

/// # Examples
/// ```
/// use::nth_field;
///
/// assert_eq!(nth_field("one, two", 0), "one");
///
/// ```
pub fn nth_field(s : &str, separator : &str, n : usize) -> String {
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
    // let dict : HashMap<String, Vec<String>> = HashMap::new();
    let keycol = 4;
    let separator = ",";

    let rdr: Box<io::BufRead> = match input.as_ref() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _   => Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };
    let mut prev = String::new();
    let mut buf  = String::new();
    let mut itr  = rdr.lines();
    // TODO: Fix issue with weird inner group line numberings
    // I'm guessing it's something related to where I introduce
    // newlines into the buffer.
    let cmd = "bash -c 'cat -n '";

    loop {
        let el = itr.next();
        match el {
            Some(el) => {
                let line = el.ok().unwrap();
                let cur = nth_field(&line, separator, keycol);
                if cur == prev || prev.is_empty() {
                    buf.push('\n'); buf.push_str(&line);
                } else {
                    exec_with_buffer(cmd, &mut buf);
                    buf.clear();
                    buf.push('\n'); buf.push_str(&line);
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
        assert_eq!(nth_field("Cool, beans, yo", ",", 0), "Cool");
    }

}
