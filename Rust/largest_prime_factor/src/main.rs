extern crate largest_prime_factor_lib;
extern crate docopt;
extern crate rustc_serialize;

const USAGE: &'static str = "
Largest prime factor.

Usage:
  primefactor.exe <n>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_n: u64
}

pub fn main() {
    use docopt::Docopt;

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    
    use largest_prime_factor_lib::largest_prime_factor;
    println!("{}", largest_prime_factor(args.arg_n));
}