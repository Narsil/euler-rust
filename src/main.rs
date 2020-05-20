use maplit::hashmap;
use std::collections::HashMap;
use std::time::Instant;
use structopt::StructOpt;

mod arithmetic;
#[macro_use]
mod combinatorics;
mod digits;
mod eleven_twenty;
mod fifty_sixty;
mod forty_fifty;
mod one_ten;
mod sixty_seventy;
mod thirty_forty;
mod twenty_thirty;

use eleven_twenty::*;
use fifty_sixty::*;
use forty_fifty::*;
use one_ten::*;
use sixty_seventy::*;
use thirty_forty::*;
use twenty_thirty::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "euler", about = "Choose problem to solve")]
struct Opt {
    /// Which problem to show solution.
    #[structopt(short, long)]
    problem_number: Option<u32>,
}
fn call(f: &dyn Fn()) {
    let now = Instant::now();
    (f)();
    println!("Time: {:?}", now.elapsed());
}

fn main() {
    let opt = Opt::from_args();
    let problems_map: HashMap<u32, &fn()> = hashmap! {
        1 => &(pb1 as fn() -> ()),
        2 => &(pb2 as fn() -> ()),
        3 => &(pb3 as fn() -> ()),
        4 => &(pb4 as fn() -> ()),
        5 => &(pb5 as fn() -> ()),
        6 => &(pb6 as fn() -> ()),
        7 => &(pb7 as fn() -> ()),
        8 => &(pb8 as fn() -> ()),
        9 => &(pb9 as fn() -> ()),
        10 => &(pb10 as fn() -> ()),
        11 => &(pb11 as fn() -> ()),
        12 => &(pb12 as fn() -> ()),
        13 => &(pb13 as fn() -> ()),
        14 => &(pb14 as fn() -> ()),
        15 => &(pb15 as fn() -> ()),
        16 => &(pb16 as fn() -> ()),
        17 => &(pb17 as fn() -> ()),
        18 => &(pb18 as fn() -> ()),
        19 => &(pb19 as fn() -> ()),
        20 => &(pb20 as fn() -> ()),
        21 => &(pb21 as fn() -> ()),
        22 => &(pb22 as fn() -> ()),
        23 => &(pb23 as fn() -> ()),
        24 => &(pb24 as fn() -> ()),
        25 => &(pb25 as fn() -> ()),
        26 => &(pb26 as fn() -> ()),
        27 => &(pb27 as fn() -> ()),
        28 => &(pb28 as fn() -> ()),
        29 => &(pb29 as fn() -> ()),
        30 => &(pb30 as fn() -> ()),
        31 => &(pb31 as fn() -> ()),
        32 => &(pb32 as fn() -> ()),
        33 => &(pb33 as fn() -> ()),
        34 => &(pb34 as fn() -> ()),
        35 => &(pb35 as fn() -> ()),
        36 => &(pb36 as fn() -> ()),
        37 => &(pb37 as fn() -> ()),
        38 => &(pb38 as fn() -> ()),
        39 => &(pb39 as fn() -> ()),
        40 => &(pb40 as fn() -> ()),
        41 => &(pb41 as fn() -> ()),
        42 => &(pb42 as fn() -> ()),
        43 => &(pb43 as fn() -> ()),
        44 => &(pb44 as fn() -> ()),
        45 => &(pb45 as fn() -> ()),
        46 => &(pb46 as fn() -> ()),
        47 => &(pb47 as fn() -> ()),
        48 => &(pb48 as fn() -> ()),
        49 => &(pb49 as fn() -> ()),
        50 => &(pb50 as fn() -> ()),
        51 => &(pb51 as fn() -> ()),
        52 => &(pb52 as fn() -> ()),
        53 => &(pb53 as fn() -> ()),
        54 => &(pb54 as fn() -> ()),
        55 => &(pb55 as fn() -> ()),
        56 => &(pb56 as fn() -> ()),
        57 => &(pb57 as fn() -> ()),
        58 => &(pb58 as fn() -> ()),
        59 => &(pb59 as fn() -> ()),
        60 => &(pb60 as fn() -> ()),
        61 => &(pb61 as fn() -> ()),
        62 => &(pb62 as fn() -> ()),
        63 => &(pb63 as fn() -> ()),
        64 => &(pb64 as fn() -> ()),
        65 => &(pb65 as fn() -> ()),
        66 => &(pb66 as fn() -> ()),
        67 => &(pb67 as fn() -> ()),
        68 => &(pb68 as fn() -> ()),
        69 => &(pb69 as fn() -> ()),
        70 => &(pb70 as fn() -> ()),
    };
    match opt.problem_number {
        Some(n) => call(problems_map.get(&n).unwrap()),
        None => {
            let mut problems: Vec<_> = problems_map.iter().collect();
            problems.sort();
            for (_n, pb) in problems {
                println!("====");
                call(pb);
            }
        }
    }
}
