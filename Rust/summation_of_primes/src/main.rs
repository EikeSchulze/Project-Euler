extern crate summation_of_primes_lib;
extern crate docopt;
extern crate rustc_serialize;

const USAGE: &'static str = "
Summation of primes below n.

Usage:
  largest_palindrome_product.exe <n>
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
    
    use summation_of_primes_lib::summation_of_primes_below;

    println!("{:?}", summation_of_primes_below(args.arg_n));
}