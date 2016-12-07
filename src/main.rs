use std::fs;
use std::io::{BufReader, BufRead, Write};

#[macro_use]
extern crate clap;
use clap::{App,Arg};

mod process;
use process::{Process, Group};

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
        .arg(Arg::with_name("skiplines")
             .long("skip")
             .help("Number of lines to skip.")
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
             .value_name("COMMAND")
             .help("The command to run on each group")
             .required(true)
             .takes_value(true)
             .index(1))
        .arg(Arg::with_name("input")
             .help("The input file to pass")
             .value_name("INPUT")
             .index(2))
        .get_matches();

    let key       = args.value_of("key").unwrap_or("1");
    let input     = args.value_of("input").unwrap_or("-");
    let separator = args.value_of("delimiter").unwrap_or(",");
    let cmd       = args.value_of("command").expect("ACH");
    let skip_n    = value_t!(args, "skiplines", usize).unwrap_or(0);
    let poolsize : usize = value_t!(args, "poolsize", usize).unwrap_or(8);
    let key_range = Range::from_list(key).unwrap();

    let rdr: Box<BufRead> = match input.as_ref() {
        "-" => Box::new(BufReader::new(std::io::stdin())),
        _   => Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    let mut prev    = String::new();
    let mut buf     = String::new();
    let mut itr     = rdr.lines();
    let mut process = Process::new(cmd, poolsize);

    // Skip initial N lines
    for _ in 0..skip_n { let _ = itr.next(); }

    loop {
        let el = itr.next();
        match el {
            Some(el) => {
                let line = el.ok().unwrap();
                let cur = linekey(&line, separator, &key_range);
                if cur == prev || prev.is_empty() {
                    buf.push_str(&line); buf.push('\n');
                } else {
                    process.push(Group::new(prev, buf.clone()));
                    buf.clear();
                    buf.push_str(&line); buf.push('\n');
                }
                prev = cur
            },
            None => {
                // TODO: Check if buf is empty first
                process.push(Group::new(prev, buf.clone()));
                buf.clear();
                break
            }
        }
    }
    for p in process.packets() {
        // TODO Print key on each line of output
        for line in p.data.unwrap().lines() {
            // Unpack printing to stdout so that we catch Broken Pipe signals from the OS
            if let Err(_) = std::io::stdout()
                .write_fmt(format_args!("{}{}{}\n", p.key, separator, line)) {
                std::process::exit(0);
            };
        }
    }
    // Sorted Output Version
    // =====================
    // let mut results : Vec<GroupProcResult> = process.packets().collect();
    // results.sort_by_key(|a| a.idx);
    // for r in results {print!("{}", r.data.unwrap());}
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
