extern crate muliples_of_3_and_5_lib;
extern crate docopt;
extern crate rustc_serialize;

const USAGE: &'static str = "
Multiples of 3 and 5.

Usage:
  multiples.exe list <n>
  multiples.exe sum <n>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_n: usize,
    cmd_list: bool,
    cmd_sum: bool
}

pub fn main() {
    use docopt::Docopt;

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.cmd_list {
        use muliples_of_3_and_5_lib::multiples_of_3_and_5_below_n;
        println!("{:?}", multiples_of_3_and_5_below_n(args.arg_n));
    }
    if args.cmd_sum {
        use muliples_of_3_and_5_lib::sum_of_multiples_of_3_and_5_below_n;
        println!("{}", sum_of_multiples_of_3_and_5_below_n(args.arg_n));
    }
}