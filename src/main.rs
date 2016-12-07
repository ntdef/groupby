use std::io;
use std::env;
use std::fs;
use std::io::*;

fn main() {
    let input = env::args().nth(1).unwrap_or(String::from("-"));

    let rdr: Box<io::BufRead> = match input.as_ref() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _   => Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    for line in rdr.lines() {
        // Rust *really* wants us to create a temporary let binding
        let line = line.unwrap();
        let v : Vec<&str> = line.split(",").collect();
        println!("{}", v[1]);
    }
}

#[cfg(test)]
mod test {

}
