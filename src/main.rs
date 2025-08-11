
use rand::Rng;
fn main() {
    println!("Bye Cruel, world!");
    let mut rng = rand::rng();
    let random_number: u32 = rng.random();
    println!("Random number: {}", random_number);


}
