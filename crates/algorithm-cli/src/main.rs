use algorithm_api::{engine_version, is_prime, sieve};
use algorithm_core::Specification;
use algorithm_api::{AlgorithmGenerator, AlgorithmGenerator as _};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("prime") => {
            let n = args.get(2).and_then(|x| x.parse::<i64>().ok()).unwrap_or(97);
            println!("{} is {}prime", n, if is_prime(n) { "" } else { "not " });
        }
        Some("sieve") => {
            let n = args.get(2).and_then(|x| x.parse::<usize>().ok()).unwrap_or(100);
            println!("{:?}", sieve(n));
        }
        Some("generate") => {
            let spec = Specification { problem: args.get(2).cloned().unwrap_or_else(|| "solve a mathematical problem".into()), goals: vec![], constraints: vec![] };
            let result = AlgorithmGenerator::generate(&spec);
            println!("{}: {}", result.name, result.steps.join(" -> "));
        }
        _ => println!("algorithm-cli v{} | commands: prime <n>, sieve <n>, generate <problem>", engine_version()),
    }
}
