// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    
    fn inspect(string: &String) {
        if string.ends_with("s") {
            println!("String is plural")
        } else {
            println!("String is singular")
        };
    }
    inspect(&arg);

    fn change(string: &mut String) {
        if !string.ends_with("s") { string.push_str("s")};
    }
    change(&mut arg);
    println!("I have many {}", arg);

    fn eat(consumes: String) -> bool {
        print!("{}", consumes);
        consumes.starts_with("b") && consumes.contains("a")
    }

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    fn add(x: &i32, y: &i32) -> i32 {
        x + y
    }
    println!("1 + 2 = {}, even via references", add(&1, &2));
}
