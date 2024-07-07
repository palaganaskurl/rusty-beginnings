pub mod music {
    pub fn play(name: String) {
        println!("Playing {}", name)
    }
}

use music::play;

fn main() {
    println!("Hello, world!");

    play("Sexy Girl".to_string());
}
