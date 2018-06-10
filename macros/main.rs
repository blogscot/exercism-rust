#![feature(trace_macros)]
#![allow(unused_variables)]
use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
  () => {
    HashMap::new()
  };

  (
     $( $key:expr => $val:expr ), *
  ) => {{
      hashmap!($($key => $val, )* )
  }};

  ( $( $key:tt => $value:tt $(,)* ), *) => {
    {
      let mut cc = HashMap::new();
      $(
          cc.insert($key, $value);
      )*
      cc
    }
  };
}

fn main() {
  trace_macros!(true);
  let s = hashmap!(
          "non-empty" => hashmap!(
           23 => 623,
           34 => 21
        ),
        "empty" => hashmap!()
  );
  trace_macros!(false);
  println!("{:?}", s);
}

// {"non-empty": {23: 623, 34: 21}}
// {"empty": {}, "non-empty": {23: 623, 34: 21}}
