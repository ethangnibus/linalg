use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        if let Ok(num) = arg.parse::<f32>() {
            let result = num / 255.0;
            println!("{}", result);
        } else {
            println!("Failed to parse argument: {}", arg);
        }
    }
}
