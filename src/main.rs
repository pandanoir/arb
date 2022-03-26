use clap::Parser;
use rand::prelude::*;
use rand::random;

#[derive(Parser, Debug)]
#[clap(name = "arb")]
#[clap(version = "0.1")]
#[clap(about = "Generate random values", long_about = None)]
struct Args {
    #[clap(help = "you can use
  - integer
  - nat
  - float
  - double
  - hexa
  - string
  - %d (alias of integer)
  - %f (alias of float)
  - %lf (alias of double)")]
    command: String,
    #[clap(long, default_value_t = 1)]
    count: u8,
}

fn replace_all<'a>(target: &str, from: &str, to: fn() -> String) -> String {
    if target.contains(from) {
        replace_all(&target.replacen(from, &to(), 1), from, to)
    } else {
        target.to_string()
    }
}

fn replace_placeholder(command: &str) -> String {
    let replacer: Vec<(&str, Box<fn() -> String>)> = vec![
        ("integer", Box::new(|| random::<i32>().to_string())),
        ("nat", Box::new(|| random::<u32>().to_string())),
        ("float", Box::new(|| random::<f32>().to_string())),
        ("double", Box::new(|| random::<f64>().to_string())),
        ("%d", Box::new(|| random::<i32>().to_string())),
        ("%f", Box::new(|| random::<f32>().to_string())),
        ("%lf", Box::new(|| random::<f64>().to_string())),
        (
            "hexa",
            Box::new(|| {
                "0123456789abcdef"
                    .chars()
                    .collect::<Vec<char>>()
                    .choose(&mut thread_rng())
                    .unwrap()
                    .to_string()
            }),
        ),
        (
            "string",
            Box::new(|| {
                ((0..20).map(|_| (thread_rng().gen_range(32..=126)) as u8 as char)).collect()
            }),
        ),
    ];
    replacer
        .into_iter()
        .fold(command.to_string(), |res, (from, to)| {
            replace_all(&res, from, *to)
        })
}

fn main() {
    let args = Args::parse();
    let command = args.command;
    let count = args.count;

    for _ in 0..count {
        println!("{}", replace_placeholder(&command))
    }
}
