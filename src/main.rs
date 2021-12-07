use minigrep::PrimaryColor;
use minigrep::SecondaryColor;
use minigrep::mix;

fn main() {
    let red = PrimaryColor::Red;
    let orange = SecondaryColor::Orange;

    mix(red, orange);
}