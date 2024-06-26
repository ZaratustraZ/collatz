// Reglas: 
// 1. Si el numero es PAR tiene que dividirse por 2
// 2. Si el numero es IMPAR tiene que multiplicarlo por 3 y sumarle 1 (3N+1)
use std::io;

fn main() -> io::Result<()> {

    println!("==========================");
    println!("    Collatz Conjecture");
    println!("==========================");
    println!("Write any number you want: ");

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line)?;

    let input: u64 = line.trim().parse().expect("Wanted a number");

    collatz(input);
    Ok(())
}

fn collatz(mut start: u64) -> Result<(), std::io::Error> {

    println!("");
    println!("");
    println!("");

    let mut max = 0;

    while start != 1 {

        if start % 2 == 0 {
            start /= 2;
            print!("{} -> ", start);
        } else {
            start = 3 * start + 1;
            print!("{} -> ", start);
        }
        if start > max {
            max = start
        }
    }
    println!("");
    println!("");
    println!("It has reached a maximum of {}", max);

    println!("");
    println!("");
    println!("");
    main()
}