#![feature(concat_idents)]
use structopt::StructOpt;

mod arithmetic;
mod eleven_twenty;
mod one_ten;
mod sixty_seventy;

pub use eleven_twenty::*;
pub use one_ten::*;
pub use sixty_seventy::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "euler", about = "Choose problem to solve")]
struct Opt {
    /// Which problem to show solution.
    #[structopt(short, long)]
    problem_number: i64,
}

fn call(n: i64) {
    match n {
        1 => pb1(),
        2 => pb2(),
        3 => pb3(),
        4 => pb4(),
        5 => pb5(),
        6 => pb6(),
        7 => pb7(),
        8 => pb8(),
        9 => pb9(),
        10 => pb10(),
        11 => pb11(),
        12 => pb12(),
        13 => pb13(),
        14 => pb14(),
        15 => pb15(),
        16 => pb16(),
        17 => pb17(),
        18 => pb18(),
        19 => pb19(),
        20 => pb20(),
        67 => pb67(),
        _ => panic!("Problem {} not solved", n),
    }
}

fn main() {
    let opt = Opt::from_args();
    call(opt.problem_number);
}
