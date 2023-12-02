use std::{
    env,
    fs::File,
    io::Read
};

mod day1;

type Day = (i32, fn(&String) -> Result<(), String>);

fn run(days: &[Day]) {
    for (day, handler) in days {
        println!("\n======== Day {} ========\n", day);

        match File::open(format!("input/day{}.txt", day)) {
            Ok(mut file) => {
                let mut input = String::new();
                file.read_to_string(&mut input).unwrap();
                match handler(&input) {
                    Ok(_) => {},
                    Err(err) => println!("Error! {}", err)
                }
            },
            Err(_) => println!("Couldn't find input file for day {}!", day)
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<Day> = vec![
        (1, day1::handler)
    ];

    println!();
    println!("░█████╗░░█████╗░░█████╗░");
    println!("██╔══██╗██╔══██╗██╔══██╗");
    println!("███████║██║░░██║██║░░╚═╝");
    println!("██╔══██║██║░░██║██║░░██╗");
    println!("██║░░██║╚█████╔╝╚█████╔╝");
    println!("╚═╝░░╚═╝░╚════╝░░╚════╝ ");
    println!("2023 - Merry Christmas! ");

    if args.len() > 1 {
        if let Ok(value) = args[1].parse::<usize>() {
            if value > 0 && value <= days.len() {
                run(&[days[value - 1]]);
            } else {
                println!("Day not found");
            }
        }
    } else {
        run(&days[..]);
    }
}