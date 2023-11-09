/*  General            
N types:     i8 i16 'i32(default) i64(long) i128(longlong), u8..128(unsigned) 'f32(float) f64(double)
             i32::MAX returns the max range/value that that size of that type can hold
Strings:     &str(like a character pointer structure or somthn)
Mutability:  Assignment are by default imutable, but can be made mutable like so: `let mut x = 4`
                Reassigning can be done but can/will/may ? delete any previous reference
Datatypes:   Primitives: Scalar Compound   (Basic types when declaring)
    Scalar:     char '!', int -4, float 2.8, boolean true/1 false/0
    Compound:   Array, Touples <fixed length, imutable> ..
        `let tup: (i8,f32,char,bool) = (2,2.1,'h',true)`
            string formatter `"{tup}"` can't directly print tup, so print elems individually
                elements referenced like tup.1 for second element
            println!("{}",tup.1) -> 2.1
        Arrays: index like so, arr[1] and need to contain elements of same type
            let arr: [type;size] = [1,2,3,4,5,6]  eg.  arr: [i8,6] = [..]
            Arrays can't be just declared, or declared as an empty, but needs to be populated from start

Preludes: functionality thats imported by default, like macros<println!> or namespaces <fn> keywrds..
Crates: Package/Library:        use std::io
Modules: Whats inside crates:   io module used for user input

Maths or computations needs to be done on same type eg. `f32+f32` works, but `f64+f32` does not.
floats need a decimal on assignments. `f32 = 4` will throw an err expecting integer. `f32 = 4.0` works
u8 244/ u8 10 = 24.4, but since they are u8, a u8 will be returned, 24.
Alternative syntax for decleration: x: u8 = 255, 255u8, 255_u8, 255 as u8;  are all equivalant
- - - - - - - - - - writing a billion: 1000000000 and 1_000_000_000 and 1e9 are all equivalant
Math operands: 4+2-1 -> 5, 3%2 -> 1 modulus.
        Numbers has methods, so eg. exponents and sqroots can be written like 
            5.pow(2) -> 25   and   x.sqrt()   and   (4+5).sqrt() -> 3
Logic operands: && || !   and evaluated in the order  1. !(not), 2. &&(and), 3. ||(or)
                            !false && true || false
                        ->  true && true || false  ->  true || false  ->  true
                            false || !true && false
                        ->  false || false && false  ->  false || false  ->  false
                            false && true || !false
                        ->  false && true || true  ->  true || true  ->  true
*/

//from the standard crate (library), use (import) module io  input/output
use std::io; // for multiple import from same crate you can write {io, convert, ..} in curly braces

fn main() {
    /* Examples of types, their formats and casting
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
    // */

    /*  Input a string to echo back
    println!("Type some!");
    let input: String = _str_input();
    println!("{input} .. ? That's all ya gotta say!? ");
    // */

    /* Alternative input structure
    let mut input = String::new();
    match io::stdin().read_line(&mut input){//.expect("failed to read line");
        Ok(_) => {
            println!("user: {}",input.trim());
        }
        Err(error) => {
            println!("error: {error}");
        }
    }
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

    /* Some functions

    fn sum2(a: i32,b: i32) -> i32 { // if return type is not specified, it will default to returning (), empty touple?
        return a + b;  
        // `return` here is reduntant, if last row is an expression, its evaluation will be the return value
        // like the expression further down, it needs to be without the ';' to actually be returned.
        // .. or just stick with `return` for clarity.
    }
    let a = _i32_input("Enter a: ");
    let b = _i32_input("Enter b: ");
    let _sum = sum2(a,b);
    let c: f32 = {             //  Assignment by multi-line expression, contained variables will be released at end of scope
        let _a:f32 = a as f32; //  Great for doing these type-convertions when no need of keeping the workings
        let _b:f32 = b as f32;
        ( _a.powf(2.0) + _b.powf(2.0) ).sqrt()  // return value on last line, MUST be without a semi-colon (;)
    };  // end of expression still needs a semi-colon
    println!("Sum of {a} + {b} = {_sum}. If a and b are cathoids of a triangle, then hypothenues c = {c}");

    // */

    /* Memory management, Heap & Stack
        // Heap and Stack are part of RAM, Random Access Memory
        // Stacks
        //  - allocation for data works like  "last in, first out"  LIFO, or FILO for  "first in, last out"
        //  - are the fastest typ of RAM and should be prioritized when possible
        //  - can only hold information with known and fixed sized, so not mutable or dynamic structures 
        //  Rust automatically removes vars from the stack when its scope reaches its end, the function in this case.
        //
        //  Stack - adds "down" removes "up"  //  Heap - works differently in Rust
        //  |  Address  |  Name  |  Value  |  //  |  Address  |  Name  |  Value  |
        //  |     0     |    x   |    2    |  //  |     5*    |    -   |   'h'
        //  |     1     |    y   |    ^    |  //  |     6     |    -   |   'e 
        //  |     2     |  mu_st |   pt5   |  //  |     7     |    -   |   'l'
        //  |     3     |    b   |   "t"   |  //  |     8     |    -   |   'l'
        //  |     4     |    -   |    f    |  //  |     9     |    -   |   'o'
        //
        // The heap uses the Memory Allocator to look up where there's enough space for new mutable data to be stored
        // Compared to the stack that just appends, this look up procedure can be muck slower
        // Stack adds pops and uses within a predictable range to backtrack
        // Heap Searches for space before storing, Then uses the pointer in stack for find and retrive data

    fn _main(){
        let _x = 2;   // <- x is added to stack
        let _y = _x;  // <- y is added to stack as a copy of x
        let mu_st = String::from("hello");  // From creates a mutable string, from the otherwise imutable "hello"
        _example_funct();
        _example_add(_x,_y);
    }                      // <- x and y and string are removed from stack. First y, str is cleared from heap, then stack, then x
    fn _example_funct(){
        let _a = "true";
        let _b = false;
    }                      // <- a and b are removed from stack. First b, then a
    fn _example_add(x:i32,y:i32) -> i32 {
        x + y  // x and y are shadows(copy's) of _x and _y
               // they get their own space on the stack, where a and b was before Their function ended and relased them
    }
    // Stack:: In: x,y,&s a,b          Out: b,a, &s,y,x 
    //  Heap:: In:     |MA-> *str      Out:      |&s -> MA.clear

    // */

    
}

// Reusable functions //

fn _str_input() -> String {
    let mut s_in = String::new();
    io::stdin().read_line(&mut s_in).expect("some error");
    let s_in = s_in; //.trim(); //.parse().unwrap();
    return s_in;
}

fn _i32_input(msg: &str) -> i32 {
    let mut input = String::new();
    if msg.len() > 0{
        println!("{msg}");
    }
    io::stdin().read_line(&mut input).expect("some error");
    let int_input: i32 = input.trim().parse().unwrap();
    return int_input;
}
