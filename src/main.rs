// Rust LC3 virtual machine

mod lc3vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("missing argument");
    }

    let filename = &args[1];
    let mut machine = lc3vm::Machine::new();
    machine.load(filename);
    machine.run();
}
