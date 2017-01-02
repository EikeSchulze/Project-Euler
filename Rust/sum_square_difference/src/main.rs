extern crate sum_square_difference_lib;
extern crate docopt;
extern crate rustc_serialize;

const USAGE: &'static str = "
Difference between the square of the sum and the sum of the squares between 1 and n.

Usage:
  sum_square_difference.exe <n>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_n: u32
}

pub fn main() {
    use docopt::Docopt;

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    use sum_square_difference_lib::sum_square_difference;
    println!("{}", sum_square_difference(args.arg_n));
}