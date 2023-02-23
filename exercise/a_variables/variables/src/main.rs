const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (ready, mut missiles): (i32, i32) = (READY_AMOUNT, STARTING_MISSILES);

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);    
}
