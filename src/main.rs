use std::io;
use std::fs;
use std::io::{BufReader,BufRead,Write,Read};
use std::process::{Command, Stdio};

#[macro_use]
extern crate clap;
use clap::{App,Arg};

mod ranges;
mod process;
use process::Process;
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

fn main() {
    let args = App::new("groupby")
        .version("1.0")
        .author("Troy de Freitas <me@ntdef.com>")
        .about("Run a command on groups of lines split by a key.")
        .arg(Arg::with_name("poolsize")
             .short("p")
             .long("pool-size")
             .help("The number of workers to allow.")
             .takes_value(true))
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
    let poolsize : usize = value_t!(args, "poolsize", usize).unwrap_or(8);
    let key_range = Range::from_list(key).unwrap();

    let rdr: Box<io::BufRead> = match input.as_ref() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _   => Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    let mut prev    = String::new();
    let mut buf     = String::new();
    let mut itr     = rdr.lines();
    let mut process = Process::new(cmd, poolsize);
    let mut work_count : usize = 0;

    loop {
        let el = itr.next();
        match el {
            Some(el) => {
                let line = el.ok().unwrap();
                let cur = linekey(&line, separator, &key_range);
                if cur == prev || prev.is_empty() {
                    buf.push_str(&line); buf.push('\n');
                } else {
                    work_count+=1;
                    process.push(buf.clone());
                    buf.clear();
                    buf.push_str(&line); buf.push('\n');
                }
                prev = cur
            },
            None => {
                work_count+=1;
                process.push(buf.clone());
                buf.clear();
                break
            }
        }
    }
    for i in 0..work_count {
        let l = process.rx.recv().unwrap();
        print!("{}", l.unwrap());
    }
}
