extern crate hamming;

fn main() {
    let result = hamming::hamming_distance("GGACG", "GGTCG").unwrap();
    print!("{}", result);
}
