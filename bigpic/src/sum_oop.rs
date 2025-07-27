#[derive(Debug)]
struct Accumulator {
    sum: i32,
}

/*
impl Accumulator {
    fn new(init: i32) -> Accumulator {
        Accumulator { sum : init }
    }
}
*/

impl Accumulator {
    fn new(sum: i32) -> Self {
        Self { sum }
    }
    fn get(self) -> i32 {
        self.sum
    }
    fn add(self, increment: i32) -> Self {
        Self {
            sum: self.sum + increment,
        }
    }
}

fn main() {
    //let acc = Acumulator { sum: 0 };
    let mut acc = Accumulator::new(0);

    for n in 3..10 {
        acc = acc.add(n);
    }

    println!("acc = {:?}", acc);
    println!("acc = {}", acc.get());
}
