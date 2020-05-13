use std::time::Instant;
use structopt::StructOpt;

mod arithmetic;
#[macro_use]
mod combinatorics;
mod digits;
mod eleven_twenty;
mod forty_fifty;
mod one_ten;
mod sixty_seventy;
mod thirty_forty;
mod twenty_thirty;

pub use eleven_twenty::*;
pub use forty_fifty::*;
pub use one_ten::*;
pub use sixty_seventy::*;
pub use thirty_forty::*;
pub use twenty_thirty::*;

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
        21 => pb21(),
        22 => pb22(),
        23 => pb23(),
        24 => pb24(),
        25 => pb25(),
        26 => pb26(),
        27 => pb27(),
        28 => pb28(),
        29 => pb29(),
        30 => pb30(),
        31 => pb31(),
        32 => pb32(),
        33 => pb33(),
        34 => pb34(),
        35 => pb35(),
        36 => pb36(),
        37 => pb37(),
        38 => pb38(),
        39 => pb39(),
        40 => pb40(),
        41 => pb41(),
        42 => pb42(),
        43 => pb43(),
        44 => pb44(),
        45 => pb45(),
        46 => pb46(),
        47 => pb47(),
        48 => pb48(),
        49 => pb49(),
        50 => pb50(),
        67 => pb67(),
        _ => panic!("Problem {} not solved", n),
    }
}

fn main() {
    let opt = Opt::from_args();
    let now = Instant::now();
    call(opt.problem_number);
    println!("Time: {:?}", now.elapsed());
}
