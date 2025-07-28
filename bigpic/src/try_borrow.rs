fn main() {
    let score = 100;

    let ref_score = &score;

    println!("{ }", ref_score);

    let mut mscore = 50;

    let mref_score = &mut mscore;

    *mref_score = 75;

    println!("{}", mref_score);
}
