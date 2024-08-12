use nes_play::nes;

type Nes<'a> = nes::Nes<'a>;

fn main() {
    let nes = Nes::new();

    println!("Hello, world!");
}
