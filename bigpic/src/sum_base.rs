fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
    // return n1+n2; // equivalent
}

fn main() {
    //let values = [8, 30, 1, 3];
    let mut values = vec![8, 30];
    values.push(1);
    values.push(3);

    let cadd = |n1: i32, n2: i32| n1 + n2;
    //let cadd = |n1, n2| n1 + n2;

    let mut sum = 0;

    /*
    for n in values {
        sum = add(sum, n);
    }
    */

    for n in &values[0..2] {
        sum = add(sum, *n);
    }

    for n in &values[2..] {
        sum = cadd(sum, *n);
    }

    println!("sum = {}", sum);

    let cum = (1..13)
        //.inspect(|n| println!("n = {} before", n))
        .filter(|n| n % 2 == 0)
        .inspect(|n| println!("n = {} after", n))
        .fold(0, |tally, n| tally + n);

    println!("cum  = {}", cum);
}
