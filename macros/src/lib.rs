#![feature(macro_at_most_once_rep)]

#[macro_export]
macro_rules! hashmap {
  () => {
    HashMap::new()
  };

  ( $( $key:expr => $value:expr $(,)? ), *) => {
    {
      let mut hm = HashMap::new();
      $(
          hm.insert($key, $value);
      )*
      hm
    }
  };
}
