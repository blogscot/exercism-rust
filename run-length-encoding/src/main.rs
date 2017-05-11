extern crate run_length_encoding as rle;

fn main() {
    // let raw: String = rle::encode("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB");
    // println!("{}", raw);
    let encoded = "12WB12W3B24WB";
    // let encoded = "12WB";
    let decoded = rle::decode(&encoded);
    println!("{}", decoded);
}
