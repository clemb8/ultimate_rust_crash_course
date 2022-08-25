const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    println!("Firing {} of my {} missiles...", READY_AMOUNT, STARTING_MISSILES);

    let _missiles: i32 = STARTING_MISSILES - READY_AMOUNT;
    println!("{} missiles left", STARTING_MISSILES - READY_AMOUNT);
}
