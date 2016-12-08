use std::io;
use std::env;
use std::fs;
use std::io::*;
use std::collections::HashMap;


fn nth_field(s : &str, n : usize) -> String {
    s.split(",").nth(n).unwrap().to_owned()
}

fn main() {
    let input = env::args().nth(1).unwrap_or(String::from("-"));
    let mut dict : HashMap<String, Vec<String>> = HashMap::new();
    let keycol = 4;

    let rdr: Box<io::BufRead> = match input.as_ref() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _   => Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    for line in rdr.lines() {
        // Rust *really* wants us to create a temporary let binding
        let line = line.unwrap();
        let key = nth_field(&line, keycol);
        let value = dict.entry(key).or_insert(Vec::new());
        value.push(line);
    }

    for k in dict.keys() {
        println!("{}|{}", k, dict[k].join("|"));
    }
}

#[cfg(test)]
mod test {

}
