use std::io;
use std::fs;
use std::io::{BufReader,BufRead};
use std::thread;

use std::sync::mpsc;
use std::process::{Command, Stdio};
use std::io::{Read, Write};

#[macro_use]
extern crate clap;
use clap::{App,Arg};

extern crate itertools;
use itertools::Itertools;

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
    let process = Process::new(cmd, poolsize);
    let lines_reader = rdr.lines();
    let itr = &lines_reader.group_by(|el| {linekey(el.as_ref().unwrap(), separator, &key_range)});
    let threads : Vec<_> = itr.into_iter().map(|(key, mut group)| {
        let el = group.map(|x| x.unwrap()).intersperse("\n".to_owned()).collect::<Vec<_>>().concat();
        thread::spawn(move || {
            let mut process =
                Command::new("sh")
                .arg("-c")
                .arg("wc -l")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            process.stdin.as_mut()
                .expect("unwrapping stdin did not work")
                .write_all(el.as_bytes()).ok();
            let _ = process.wait();
            let mut bufout = String::new();
            process.stdout.as_mut().unwrap().read_to_string(&mut bufout).ok();
            print!("{}", bufout);
        })
    }).collect();
    for t in threads {
        let _ = t.join();
    }
}

// SEE: http://stackoverflow.com/questions/28599334/how-do-i-run-parallel-threads-of-computation-on-a-partitioned-array
//
// fn main() {
//     let numbers = random_vec(10);
//     let num_tasks_per_thread = numbers.len() / NTHREADS;

//     // The `collect` is important to eagerly start the threads!
//     let threads: Vec<_> = numbers.chunks(num_tasks_per_thread).map(|chunk| {
//         thread::scoped(move || {
//             chunk.iter().cloned().sum()
//         })
//     }).collect();

//     let thread_sum: i32 = threads.into_iter().map(|t| t.join()).sum();
//     let no_thread_sum: i32 = numbers.iter().cloned().sum();

//     println!("global sum via threads    : {}", thread_sum);
//     println!("global sum single-threaded: {}", no_thread_sum);
// }
