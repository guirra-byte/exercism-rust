pub mod acronym;
pub mod reverse_string;
pub mod chessboard_grains;
pub mod beer_song;
pub mod sum;

fn main() {
    println!("Studying Rust...");
    acronym::main();
    reverse_string::main();
    chessboard_grains::main();
    beer_song::main();
    sum::main();

}

// 1 Byte === 8 Bits;
// 8 Bytes === 256 Bits;
// i8 -> i128 && f8 -> f128 && u8 -> u128; -> Capacidade de Bits de armazenamento;
// u8 === unsigned -> 0 to 255;
// i8 === signed -> -128 to 127;
