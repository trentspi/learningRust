fn main() {

  let farray = [3.5,1.0,2.0,4.0123,4.00];

  println!("farray has {} elements", farray.len());

  let foo_array: [f32; 5] = [1.0,2.0,3.0,4.0,5.0];
  let init_array: [f32; 5] = [2.0; 5];

  println!("foo array first print");
  for x in foo_array.iter() {
    println!("{}", x)
  }
  
  println!("same foo array, but skips first 2 elements and takes first 2 elements");
  for x in foo_array.iter().skip(2).take(2) {
    println!("{}", x);  
  }
}
