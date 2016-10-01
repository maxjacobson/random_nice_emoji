extern crate pick_one;
use pick_one::pick_one_str;

fn main() {
    // cool, friendly emoji that look fine against a black terminal background
    let list = ["🐖", "😅", "🌸", "🐙", "🎑", "🖌", "☕", "📊", "🐋", "🌈",
                "✨"];

    let emoji = pick_one_str(&list);
    print!("{}", emoji);
}
