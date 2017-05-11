extern crate run_length_encoding as rle;

fn main() {
    let encoded: String = rle::encode("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB");
    println!("{}", encoded);
    let decoded = rle::decode(&encoded);
    println!("{}", decoded);
}
