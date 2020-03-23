mod repl;

fn main() {
    println!("teal-lang");
    match repl::start() {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    }
}
