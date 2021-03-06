use std::mem;

const LINESIZE:i8 = 1024;

fn main() {

  let a1 = -78912312;
  let a2:i16 = 2;
  let mut a3:u32 = 123;
  let mut a4:f32 = -123.321;

  println!("The size of a1 is {} bytes", mem::size_of_val(&a1));

  let ch= 'S';
  let str = "BANANA";
  println!("Taking BANANA and adding S to the end length of str times");
  for x in str.chars() {
    println!("For loop iteration character: {}",x);
    println!("{}{}",str,ch);
  }
}
