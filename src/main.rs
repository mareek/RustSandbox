extern crate rand;

use rand::Rng;

fn main() {
    print_random_chars(15);
    println!("");
}

fn print_random_chars(nb_chars: u32) {
    let mut rng = rand::thread_rng();
    let corpus = ['m', 'a', 't', 't', 'h', 'i', 'e', 'u'];
    for _i in 0..nb_chars {
        print!("{}", rng.choose(&corpus).unwrap());
    }
}

fn generate_random_int(max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, max)
}
