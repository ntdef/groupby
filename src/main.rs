use std::io;
use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
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
    let mut dict : HashMap<String, Vec<String>> = HashMap::new();
    let keycol = 2;

    let rdr: Box<io::BufRead> = match input.as_ref() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _   => Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    // TODO Convert to while loop buffered read.
    // See: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
    for line in rdr.lines() {
        // Rust *really* wants us to create a temporary let binding
        let line = line.unwrap();
        let key = nth_field(&line, keycol);
        let value = dict.entry(key).or_insert(Vec::new());
        value.push(line);
    }

    for k in dict.keys() {
        let mut cmd = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::piped()).spawn().unwrap();
        // println!("{}|{}", k, dict[k].join("|"));
        cmd.stdin.as_mut().unwrap().write_all(dict[k].join("\n").as_bytes()).ok();
        let out = cmd.wait_with_output().unwrap().stdout;
        print!("{}", String::from_utf8_lossy(&out));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(nth_field("Cool, beans, yo", 0), "Cool");
    }

}
