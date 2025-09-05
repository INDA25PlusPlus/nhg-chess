// name of library is defined in cargo.toml
use nhg_chess::echo;

fn main() {
    let input = "hello, world!";
    let output = echo(input);
    println!("{}", output);
}
