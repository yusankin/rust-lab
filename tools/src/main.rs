mod generator;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <category> <problem_name>");
        return;
    }

    let category = &args[1];
    let problem_name = &args[2];

    generator::generate(category, problem_name);
}
