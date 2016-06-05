extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn main() {
    // cool, friendly emoji that look fine against a black terminal background
    let list = vec!["🐖", "😅", "🌸", "🐙", "🎑", "🖌", "☕", "📊", "🐋"];
    let between = Range::new(0, list.len());
    let mut rng = rand::thread_rng();
    let index = between.ind_sample(&mut rng);
    let emoji = list[index];
    print!("{}", emoji);
}
