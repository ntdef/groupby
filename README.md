# groupby

Parallel group-by-apply in the shell, powered by Rust.

`groupby` is a small CLI tool for splitting up a stream by a key and running an operation on each group. It's inspired by `xargs` and `GNU Parallel`, but for discrete groups.
`groupby` will try to read input as a stream as needed, usally using a smaller memory footprint than Python's pandas or R's dataframes. It works great for
[data science at the command line](https://www.datascienceatthecommandline.com/1e/).

## Installation

You can build and install `groupby` from source using `cargo`. Make sure you have `cargo` installed. Then run:

``` sh
git clone https://github.com/ntdef/groupby
cd groupby
cargo install --path .
```


## Usage

``` txt
groupby 1.0
Troy de Freitas
Run a command on groups of lines split by a key.

USAGE:
    groupby [OPTIONS] <COMMAND> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delimiter <DELIMITER>    The field separator.
    -k, --key <KEYCOLS>            Sets a custom config file
    -p, --pool-size <poolsize>     The number of workers to allow.
        --skip <skiplines>         Number of lines to skip.

ARGS:
    <COMMAND>    The command to run on each group
    <INPUT>      The input file to pass
```


## Examples

Count the number of lines in each group of a CSV, keyed by the first column:

``` sh
groupby -d ',' -k 1 'wc -l' input.csv
```


Same as before, but use 4 parallel jobs:

``` sh
groupby -p 4 -d ',' -k 1 'wc -l' input.csv
```

## Contributing

Contributions are welcome! However, I can't guarantee that I'll get back to you in a timely fashion. The smaller the patch, the better! If you would like to make a significant contribution (~ more than 20 lines of code), please create an issue first before starting any work.

## License

Copyright © 2021 Troy de Freitas

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
