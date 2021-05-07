use rand::Rnd;

fn main() {
    let x: u8 = rand::random();
    println!("{}", x);

    let y = rand::random::<f64>();
    println!("{}", y);

    // if rand::random() { // generates a boolean
    // println!("Heads!");

}
