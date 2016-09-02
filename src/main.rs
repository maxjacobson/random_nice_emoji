extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn sample(list: Vec<&str>) -> &str {
    let between = Range::new(0, list.len());
    let mut rng = rand::thread_rng();
    let index = between.ind_sample(&mut rng);
    let el = list[index];
    return el;
}

fn main() {
    // cool, friendly emoji that look fine against a black terminal background
    let list = vec!["🐖", "😅", "🌸", "🐙", "🎑", "🖌", "☕", "📊", "🐋", "🌈",
                    "✨"];
    let emoji = sample(list);
    print!("{}", emoji);
}
