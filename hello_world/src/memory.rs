fn main() {
  let mut v = vec![];

  v.push("Hello");

  let x = &v[0].clone();

  v.push("World");

  println!("{}", x);
}
