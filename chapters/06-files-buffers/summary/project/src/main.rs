mod lib;

fn main() {
    let input = "hello, nvim";
    match lib::process(input) {
        Ok(out) => println!("ok: {}", out),
        Err(e) => eprintln!("err: {}", e),
    }
}
