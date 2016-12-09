use std::io;
use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::io::Read;
// use std::error::Error;
// use std::io::stdout;
use std::collections::HashMap;
use std::process::{Command, Stdio};

/// # Examples
/// ```
/// use::nth_field;
///
/// assert_eq!(nth_field("one, two", 0), "one");
///
/// ```
pub fn nth_field(s : &str, n : usize) -> String {
    s.split(",").nth(n).unwrap().to_owned()
}

fn main() {
    let input = env::args().nth(1).unwrap_or(String::from("-"));
    // let dict : HashMap<String, Vec<String>> = HashMap::new();
    let keycol = 4;

    let mut rdr: Box<io::BufRead> = match input.as_ref() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _   => Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    // TODO Convert to while loop buffered read.
    // See: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line

    // let mut buffer = String::new();
    // while rdr.read_line(&mut buffer).unwrap() > 0 {
    //     // work with buffer
    //     println!("{:?}", buffer);
    //     buffer.clear();
    // }

    let mut prev = "".to_owned();
    let mut buf = String::new();
    let mut bufout = String::new();

    for line in rdr.lines() {
        // Rust *really* wants us to create a temporary let binding
        let line = line.unwrap();
        let curkey = nth_field(&line, keycol);
        if curkey != prev {
            let cmd = Command::new("bash")
                .arg("-c")
                .arg("wc -l")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            cmd.stdin.unwrap()
                .write_all(buf.as_bytes()).ok();
            cmd.stdout.unwrap()
                .read_to_string(&mut bufout).ok();
            print!("{}", bufout);
            bufout.clear(); buf.clear();
        }
        prev = curkey;
        buf.push('\n');
        buf.push_str(&line);
        // let value = dict.entry(cur).or_insert(Vec::new());
        // value.push(line);
    }
    let cmd = Command::new("bash")
        .arg("-c")
        .arg("wc -l")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    cmd.stdin.unwrap()
        .write_all(buf.as_bytes()).ok();
    cmd.stdout.unwrap()
        .read_to_string(&mut bufout).ok();
    print!("{}", bufout);
    bufout.clear(); buf.clear();
    // println!("{} | {}", prev, buf);

    // for k in dict.keys() {
    //     let mut cmd = Command::new("wc")
    //         .arg("-l")
    //         .stdin(Stdio::piped()).spawn().unwrap();
    //     // println!("{}|{}", k, dict[k].join("|"));
    //     cmd.stdin.as_mut().unwrap().write_all(dict[k].join("\n").as_bytes()).ok();
    //     let out = cmd.wait_with_output().unwrap().stdout;
    //     print!("{}", String::from_utf8_lossy(&out));
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(nth_field("Cool, beans, yo", 0), "Cool");
    }

}
