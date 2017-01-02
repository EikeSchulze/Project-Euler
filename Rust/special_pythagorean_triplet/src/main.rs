extern crate special_pythagorean_triplet_lib;
extern crate docopt;
extern crate rustc_serialize;

const USAGE: &'static str = "
Special Pythagorean triplet.

Usage:
  special_pythagorean_triplet.exe <n>
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
    
    use special_pythagorean_triplet_lib::special_pythagorean_triplet_product;
    println!("{:?}", special_pythagorean_triplet_product(args.arg_n));
}