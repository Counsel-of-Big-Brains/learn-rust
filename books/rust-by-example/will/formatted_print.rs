use std::fmt;

fn main() {
  println!("{} days", 31);

  println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

  println!("{subject} {verb} {object}",
          object="the lazy dog",
          subject="the quick brown fox",
          verb="jumped over");

  println!("{} out of every {:b} people know binary, the other one doesn't", 1, 2);

  println!("{number:>width$}", number=1, width=6);
  println!("{number:0>width$}", number=1, width=6);
  println!("{:>1$}", 1, 6);

  println!("My name is {0}, {1} {0}", "Bond", "James");

  #[allow(dead_code)]
  struct Structure(i32);
  impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "<Structure({})>", self.0)
    }
  }
  println!("This struct `{}` won't print, yet!", Structure(3));

  // Works in Rust 1.58 and above.
  // let number: f64 = 1.0;
  // let width: usize = 6;
  // println!("{number:>width$}");

  println!("PI is roughly {pi:.3}", pi=3.141592);
}
