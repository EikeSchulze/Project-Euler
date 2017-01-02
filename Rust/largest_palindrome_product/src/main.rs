extern crate largest_palindrome_product_lib;
extern crate docopt;
extern crate rustc_serialize;

const USAGE: &'static str = "
Largest palindrome product of two numbers below n.

Usage:
  largest_palindrome_product.exe palindrome <n>
  largest_palindrome_product.exe largest <n>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_n: usize,
    cmd_palindrome: bool,
    cmd_largest: bool
}

pub fn main() {
    use docopt::Docopt;

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.cmd_palindrome {
        use largest_palindrome_product_lib::is_palindrome;
        println!("{}", is_palindrome(args.arg_n as u64));
    } else if args.cmd_largest {
        use largest_palindrome_product_lib::largest_palindrome_product;
        println!("{:?}", largest_palindrome_product(args.arg_n));
    }
}