fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

/* Shadowing - Variable replacement and back to original.
fn main() {
    let x = 5;

        let x = x + 1;

   {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
   }

    println!("The value of x is: {x}");
}
*/

/* Positive and Negative Numbers
 Length	  Signed	    Unsigned      Signed Integers (Signed = -X; Unsigned = Assumed to be positive)
 8-bit	  i8	        u8
 16-bit	  i16	        u16
 32-bit	  i32	        u32
 64-bit	  i64	        u64
 128-bit	  i128	        u128
 arch	  isize	        usize
*/

/* Numbers 
 Number literals	        Example       
 Decimal	                98_222
 Hex	                    0xff
 Octal	                0o77
 Binary	                0b1111_0000
 Byte                    (u8 only)	b'A'
*/

/* Floating-Point Types (f64 & f32)
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
*/

/* Basic Mathemtics Operators 
fn main() {
    addition
        let sum = 5 + 10;
    subtraction
        let difference = 95.5 - 4.3;
    multiplication
        let product = 4 * 30;
    division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1
    remainder
        let remainder = 43 % 5;
    }
*/

/* Booleans 
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
 */

/* Char Types
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
 */

/* Tuple and how to access each value - values can be of different types
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
*/

/* Array - MUST have the same type - fixed length 
#![allow(unused)]
fn main() {
let a: [i32; 5] = [1, 2, 3, 4, 5];
}

We are creating the array "a", which contains 5 instances of the number 3 [3; 5]
let a = [3; 5]; = let a = [3, 3, 3, 3, 3];

Accessing data in the array
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
*/
