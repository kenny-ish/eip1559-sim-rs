use eip1559_sim_rs::simulate;

fn arg(args: &[String], name: &str, default: f64) -> f64 {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let start = arg(&args, "--base", 10.0);
    let shock = arg(&args, "--shock-blocks", 20.0) as usize;
    let blocks = arg(&args, "--blocks", 60.0) as usize;
    let path = simulate(start, shock, blocks);
    println!("{:>5} {:>12} {:>7}", "block", "base (gwei)", "full");
    for (i, (base, full)) in path.iter().enumerate() {
        println!("{:>5} {:>12.4} {:>6.0}%", i + 1, base, full * 100.0);
    }
    match path.iter().enumerate().skip(shock).find(|(_, p)| (p.0 - start).abs() / start < 0.05) {
        Some((i, _)) => println!("\nback within 5% of {start} gwei after block {}", i + 1),
        None => println!("\nnot back within 5% after {blocks} blocks"),
    }
}
