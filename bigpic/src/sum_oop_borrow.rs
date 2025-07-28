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
    //immutable borrow -> lends the reference (like lend a book ), can read not modify
    fn get(&self) -> i32 {
        self.sum
    }
    //mutable borrow -> lends the reference (like lend a board), one can write at time
    fn add(&mut self, increment: i32) {
        self.sum += increment;
    }
    //move -> performs a copy of the elements (transfers ownership)
    /*
    fn combine(acc1: Accumulator, acc2: Accumulator) -> Accumulator {
        Accumulator::new(acc1.sum + acc2.sum)
    }
    */
    fn combine(acc1: Self, acc2: Self) -> Self {
        Self::new(acc1.sum + acc2.sum)
    }
}

fn main() {
    //let acc = Acumulator { sum: 0 };
    let mut acc = Accumulator::new(0);

    for n in 3..10 {
        acc.add(n);
    }

    println!("acc = {:?}", acc);
    println!("acc = {}", acc.get());

    let acc1 = Accumulator::new(1);
    let acc2 = Accumulator::new(2);

    let result = Accumulator::combine(acc1, acc2);

    println!("accs= {}", result.get());

    let mut evens_acc = Accumulator::new(0);
    let mut odds_acc = Accumulator::new(0);

    for n in 3..10 {
        if n % 2 == 0 {
            evens_acc.add(n);
        } else {
            odds_acc.add(n);
        }
    }

    // Atention here: you define a new accumulator based on the former ones in this case
    // not borrowing but instead transfering ownership
    let acctot = Accumulator::combine(evens_acc, odds_acc);
    println!("acct = {}", acctot.get());
    //println!("even_acc {}", evens_acc.get());// will fail because evens_acc is moved
}
