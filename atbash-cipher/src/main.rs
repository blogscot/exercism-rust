extern crate atbash_cipher as cipher;

fn main() {
    let result = cipher::encode("Testing, testing.");
    println!("{:?}", result);

}
