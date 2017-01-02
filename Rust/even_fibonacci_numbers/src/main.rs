extern crate even_fibonacci_numbers_lib;
extern crate docopt;
extern crate rustc_serialize;

const USAGE: &'static str = "
Even fibonacci numbers.

Usage:
  fibonacci.exe list <n>
  fibonacci.exe list even <n>
  fibonacci.exe sum <n>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_n: usize,
    cmd_list: bool,
    cmd_even: bool,
    cmd_sum: bool
}

pub fn main() {
    use docopt::Docopt;

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.cmd_list && args.cmd_even {
        use even_fibonacci_numbers_lib::list_even_fibonacci_numbers_below;
        println!("{:?}", list_even_fibonacci_numbers_below(args.arg_n));
    } else if args.cmd_list && !args.cmd_even {
        use even_fibonacci_numbers_lib::list_fibonacci_numbers_below;
        println!("{:?}", list_fibonacci_numbers_below(args.arg_n));
    } else if args.cmd_sum {
        use even_fibonacci_numbers_lib::sum_even_fibonacci_numbers_below;
        println!("{}", sum_even_fibonacci_numbers_below(args.arg_n));
    }
}