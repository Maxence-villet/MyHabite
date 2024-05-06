
pub mod Summon;

fn main() {
    let summon = Summon::Summon::new(1200);
    println!("{}", summon.get_gem_needed());
    println!("Hello, world!");
}
