extern crate smallest_multiple_lib;
extern crate docopt;
extern crate rustc_serialize;

const USAGE: &'static str = "
Smallest multiple of all numbers below n.

Usage:
  smallest_multiple.exe <n>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_n: usize
}

pub fn main() {
    use docopt::Docopt;

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    use smallest_multiple_lib::smallest_multiple;
    println!("{}", smallest_multiple(args.arg_n));
}