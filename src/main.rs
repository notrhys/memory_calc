use std::process::{exit, Command};

fn main() {
    if !cfg!(target_os = "linux") {
        println!("this only works on linux");
        exit(0);
    }

    let output = Command::new("free")
        .arg("-h")
        .output()
        .expect("Failed to execute free command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();

    let mem_line = lines.get(1).expect("Failed to get memory line");
    let mem_values: Vec<&str> = mem_line.split_whitespace().collect();

    println!("total: {}", mem_values[1]);
    println!("used: {}", mem_values[2]);
    println!("available: {}", mem_values[3]);
    println!("free: {}", mem_values[4]);
    println!("cache: {}", mem_values[5]);
    println!("shared: {}", mem_values[6]);

    let total_ram = fix_value(mem_values[1]);
    let free_ram = fix_value(mem_values[3]);
    let shared_ram = fix_value(mem_values[4]);
    let cache_ram = fix_value(mem_values[5]);
    let available_ram = fix_value(mem_values[6]);

    let value = available_ram + free_ram + shared_ram + (0.50 * cache_ram);
    let final_value = total_ram - value;
    let round = (final_value * 10.0).round() / 10.0;

    println!("\nserver is currently using: {}Gi", round.abs());
}

fn fix_value(input: &str) -> f64 {
    let fixed = input
        .replace("Gi", "")
        .replace("Mi", "");

    if input.contains("Mi") {
        let to_value = fixed.parse::<f64>().unwrap();
        to_value / 1024.0
    } else {
        fixed.parse::<f64>().unwrap()
    }
}