/// Generates a Pythagorean soltuion for the given side a
/// http://www.friesian.com/pythag.htm
fn calculate_sides(a: u32) -> (u32, u32, u32) {
  let is_even = |x| x % 2 == 0;
  if is_even(a) {
    let b = (a / 2).pow(2) - 1;
    (a, b, b + 2)
  } else {
    let b = (a.pow(2) - 1) / 2;
    (a, b, b + 1)
  }
}

fn triplet(n: u32, sides: (u32, u32, u32)) -> (u32, u32, u32) {
  let (x, y, z) = sides;
  let a = x * n;
  let b = y * n;
  let c = z * n;
  assert_eq!(a * a + b * b, c * c);
  (a, b, c)
}

pub fn find() -> Option<u32> {
  for m in 3.. {
    let sides = calculate_sides(m);
    for n in 1.. {
      let (a, b, c) = triplet(n, sides);
      let sum = a + b + c;
      if sum == 1000 {
        return Some(a * b * c);
      } else if sum > 1000 {
        break;
      }
    }
  }
  None
}
