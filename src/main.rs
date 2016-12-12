use std::io;
use std::fs;
use std::io::{BufReader,BufRead,Write,Read};
use std::process::{Command, Stdio};
extern crate clap;
use clap::{App,Arg};

mod ranges;
use ranges::Range;

fn linekey(s : &str, separator : &str, indices : &[Range]) -> String {
    let mut fields = s.split(separator);
    let mut bufout = String::new();
    let mut prev : usize = 0;
    for &Range { low, high } in indices.iter() {
        if high - low > 0 {
            for i in low..high {
                let s = fields.nth(i - prev - 1)
                    .expect("Not enough columns");
                bufout.push_str(s);
                prev = i;
            }
        } else {
            let s = fields.nth(high - prev - 1)
                .expect("Not enough columns");
            bufout.push_str(s);
            prev = high;
        }
    }
    bufout
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
    let args = App::new("groupby")
        .version("1.0")
        .author("Troy de Freitas <me@ntdef.com>")
        .about("Run a command on groups of lines split by a key.")
        .arg(Arg::with_name("key")
             .short("k")
             .long("key")
             .value_name("KEYCOLS")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("delimiter")
             .short("d")
             .long("delimiter")
             .value_name("DELIMITER")
             .help("The field separator.")
             .takes_value(true))
        .arg(Arg::with_name("command")
             .short("e")
             .value_name("COMMAND")
             .help("The command to run on each group")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("input")
             .help("The input file to pass")
             .value_name("INPUT")
             .index(1))
        .get_matches();

    let key       = args.value_of("key").unwrap_or("1");
    let input     = args.value_of("input").unwrap_or("-");
    let separator = args.value_of("delimiter").unwrap_or(",");
    let cmd       = args.value_of("command").expect("ACH");
    let key_range = Range::from_list(key).unwrap();
    // let cmd = "bash -c 'cat -n '";

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
                let cur = linekey(&line, separator, &key_range);
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
                buf.clear();
                break
            },
        }
    }
}
