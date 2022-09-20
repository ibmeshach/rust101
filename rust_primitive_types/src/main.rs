
// 1. To create a new project in rust

        /* cargo new filename */
        


// 2. you can't overwrite an immutable variable

        /*
        immutable(){
         let x = 5;
         x = 6;
        }

        */

        fn overwrite(){
            let x = 6;
            let x = 5;
        }

        // this is acceptable



// 3. You can't mutate a variable's type

        /*
        fn mutate_type(){
            let some_string = "donut";
            some_string = some_string.len();
        }
        */

       

        // some_string.len() will return an integer and assigning an interger to an already declared variable of type string slice
        // this will give an error


// 4. If you must mutate the type, all you have to do is use the let keyword again

        fn overwrite_type(){
            let some_string = "donut";
            let some_string = some_string.len();
        }

        // this is accepted
        // let is how you tell rust to set a new varable and if you use the same variable name, it is going to overwrite


// 5. Declearing Data types

        fn declare(){
            let x:u32 = 20;
        }

        // in rust you have the option to declare your data type or not, it is advisable to
        // u32 is one of the integer types provided by rust
        // intergers can either be signed or unsigned




// 6. Integers

        fn integers(){
            let signed_int:i64 = 34;
            let unsigned_int:u32 = 11;
        }

        // integers ranges from 8-128
        // u8, u16, u32, u64, u128
        // i8, i16, i32, i64, i128


// 7. Booleans

        fn booleans(){
            let t:bool = true;
            let f:bool = false;
        }

        // rust uses the lower case for true and false



//  8. Strings

        // strings decleared without any object(raw string) is called a slice. You can also decleare it with the data types

        fn strings(){
            let some_slice: &str = "donut"; // slice
            
            let some_string: String = String::from("donut");
        }



// 9. Constants

        // where you declear your constants determines the scope of the constant. 
        // If you don't declear it in a function, it is available to the rest of the program

        const SOME_CONSTANT: u32 = 20;

fn main() {
   println!("Primitive Types in Rust")
}
