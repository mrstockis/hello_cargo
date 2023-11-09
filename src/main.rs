
/*                         i32::MAX returns the max range/value that type can hold
N types:     i8 i16 'i32(default) i64(long) i128(longlong), u8..128(unsigned) 'f32(float) f64(double)
Strings:     &str(like a character pointer structure or somthn)
Mutability:  Assignment are by default imutable, but can be made mutable like so: `let mut x = 4`
                Reassigning can be done but can/will/may ? delete any previous reference
Datatypes:   Primitives: Scalar Compound   (Basic types when declaring)
    Scalar:     char '!', int -4, float 2.8, boolean true/1 false/0
    Compound:   Array, Touples <fixed length, imutable> ..
        let tup: (i8,f32,char,bool) = (2,2.1,'h',true)
            string formatter "{}" can't directly print tup, so print elems individually
                elements referenced like tup.1 for second element (like `tup[1]`)
            println!("{}",tup.1) -> 2.1       
        Arrays: index like so, arr[1] and need to contain elements of same type
            let arr: [type;size] = [1,2,3,4,5,6]  eg.  arr: [i8,6] = [..]
            arrays can't be just declared, or declared as an empty. Needs to be populated from start 
Preludes: functionality thats imported by default, like macros<println!> or namespaces <fn> keywrds..
Crates: Package/Library:        use std::io
Modules: Whats inside crates:   io module for user input

Maths or computations needs to be done on same type eg. `f32+f32` works, but `f64+f32` does not.
floats need a decimal on assignments. `f32 = 4` will throw an err expecting integer. `f32 = 4.0` works
u8 244/ u8 10 = 24.4, but since they are u8, a u8 will be returned, 24.
Alternative syntax for decleration: x: u8 = 255, 255u8, 255_u8, 255 as u8;  are all equivalant
- - - - - - - - - - writing a billion: 1000000000 and 1_000_000_000 and 1e9 are all equivalant
Math operands: 4+2-1 -> 5, 3%2 -> 1 modulus, 
Logic operands: && || !   and evaluated in the order  1. !(not), 2. &&(and), 3. ||(or)
                            !false && true || false
                        \\  true && true || false  ->  true || false  ->  true
                            false || !true && false
                        \\  false || false && false  ->  false || false  ->  false
                            false && true || !false
                        \\  false && true || true  ->  true || true  ->  true
*/

//from standard crate (library), use (import) module io  input/output
use std::io; // for multiple import from same crate you can write {io, convert, ..} in curly braces

fn main() {
    const THIS_IS_A_CONSTANT: i8 = 4;

    let s: &str = "Hello World!";
    let x: i32 = 200;
    let x = x + 4;

    let mut tup: (&str, i32, i8, bool) = (s, x, THIS_IS_A_CONSTANT, 1 == 0) ;
    // tup can be made mutable to change values, but datatypes and size (tup.length) persist
    tup.0 = "hi";

    let _arr1 = [1,2,3,4,5,6,7];  // [i32; 7]
    //let mut arr2: [i8;8] = [];
    //arr2[2] = 4; 

    // convert types, casting can only be made on primitives
    let _x: i32 = 4; // the underscore prefix makes makes compiler ignore `not being used` warnings
    let _y: i8 = x as i8; // like casting I think. Only needed when 'schrinking' it seems
                            // x.into() does the same, if you can't remember what type y was

    println!("{}",1.32e6);
    println!("{}!, x = {}, constant = {}", s, x,THIS_IS_A_CONSTANT);
    println!("tup: {}, {}, {}, {}",tup.0,tup.1,tup.2,tup.3);

    //*  Input a string to echo back  
    println!("Type some!");
    let input: String = _str_input();
    println!("{input} .. ? That's all ya gotta say!? ");
    
    //let mut input = String::new();
    //match io::stdin().read_line(&mut input){//.expect("failed to read line");
    //    Ok(_) => {
    //        println!("user: {}",input.trim());
    //    }
    //    Err(error) => {
    //        println!("error: {error}");
    //    }
    //}
    // */

    /*  Input a number to convert and calculate with
    let mut input = String::new();
    println!("Some maths! add a number to 5: ");
    io::stdin().read_line(&mut input).expect("some error");
    
    let int_input: i64 = input.trim().parse().unwrap();
    // trim removes trailing character from input when user presses `Enter`
    // parse attempts to convert the String to the requested format
    // unwrap returns the actual integer value, rather than the full response given by parse
    println!("Input was {0}, 5+{0}={1} ",int_input,5+int_input);
    // */
    
    
    
    /*  Syntax for `if, else if, else`, and match(switch) statements
    let foods: [&str;3] = ["cookie","potato","..ehh.."];
    
    println!("Pick a food!");
    let mut i: u8 = 1;
    for f in foods {
        println!("{i}: {f}");
        i += 1;
    }

    
    //let mut input: String = String::new();
    //io::stdin().read_line(&mut input).expect("you failed");
    //let int_inp: i8 = input.trim().parse().unwrap();
    let int_inp: i32 = _i32_input();

    let food:&str = match int_inp {
        1 => foods[0],
        2 => foods[1],
        3 => foods[2],
        _ => "Not an option, mate!"
    };

    if food == "cookie"{
        println!("Yay, a {food} :D");
    } else if food == "potato" {
        println!("Noo, {food} D:");
    } else if food == "..ehh.." {
        println!("WHAT IS THAT!?");
    } else {
        println!("{food}");
    }
    // */
    

    //* Next tutorial
    
    // */

}


// Reusable functions

fn _str_input() -> String {
    let mut s_in = String::new();
    io::stdin().read_line(&mut s_in).expect("some error");
    let s_in = s_in; //.trim(); //.parse().unwrap();
    return s_in;
}

fn _i32_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("some error");
    let int_input: i32 = input.trim().parse().unwrap();
    return int_input;
}