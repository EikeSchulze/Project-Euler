extern crate nth_prime_lib;
extern crate docopt;
extern crate rustc_serialize;

const USAGE: &'static str = "
Calculate the nth prime.

Usage:
  nth_prime.exe <n>
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
    use nth_prime_lib::nth_prime;
    println!("{}", nth_prime(args.arg_n));
}