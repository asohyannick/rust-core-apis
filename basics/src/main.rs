fn main() {
   let first_name = "Alice";  // immutable by default (like Java's final)
   let last_name = "Smith";  // immutable by default (like Java's final)
   let age = 42;
   let pi = 3.14;

   // To make it mutable, you must say so explicitly
   let mut counter = 0;
   counter += 1;

   println!("{} {} {} {} {}", last_name, first_name, age, pi, counter);

   let x = 5;
   let x = x + 1;  // shadows the previous x
   let x = x * 2; // shadows again
   println!("{}", x); // prints 12

   let input = "42";
   let input: i32 = input.parse().unwrap();
   print!("{}", input);
   /*
     signed integers
     i8,
     i16,
     i32,
     i64,
     i128

     unsigned integers
     u8,
     u16,
     u32,
     u64,
     u128

     pointer-sized, used for indexing

     isize,
     usize

     floating point
     f32, f64

     bool -> true or false
     char
   */

  let new_age = 30;
  let temperature = 36.6;
  let is_active = true;
  let letter = 'R';
  print!("{my_age} {temp} {is_act} {ltr}", 
  my_age = new_age, 
  temp = temperature, 
  is_act = is_active, 
  ltr = letter
)

}
