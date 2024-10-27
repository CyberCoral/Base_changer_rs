/*
Project name: Base_changer_rs
Author: CyberCoral
Date of creation: 26 / 10 / 2024
--------------------------------
Github:
https://github.com/CyberCoral

--------------------------------
Description of program:
It converts a number (in base 10),
then it gets transformed into the
specified base and, finally, to the
final base.

*/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]


// With this, you can read arguments 
// from the command line prompt. 
use std::env;
use std::io;
use std::io::Write;

const base1_case1 : u64 = 10;
// Number is, by default, base 10.
fn base_converter_case1(num1 : u64, base2 : u64) -> Vec<u64> {
    // Edge cases of base_conversion.
    // Insert them if the program doesn't cover them already.
    if base2 < 2{
        println!("Base2 cannot be lower than 2.");
        std::process::exit(0);
    };

    // Vector and unsigned int definitions.
    let mut num_vect : Vec<u64> = Vec::new();
    let mut num_original : u64 = num1;

    // Obtain the number as a vector with u64 integers.
    while num_original != 0{
        //println!("{} is num_original",num_original);
        if num_original == 0{
            break;
        }
        num_vect.insert(0,(num_original % base1_case1).try_into().unwrap());
        num_original /= base1_case1;
    };

    // Define a "constant" so it has that value stored.
    let LNV : u64 = (num_vect.len() as u64); 
                                      

    // Makes sure the data is correct.
    for data in 0..LNV{
        if (num_vect[(data as usize)] as u64) > base1_case1 {
            println!("The digit {} is greater than the base.", data);
            ()
        }
    };

    // Obtains the real value of the num1.
    for i in 0..LNV{
        num_original += (num_vect[(i as usize)] * base1_case1.pow((LNV - ((i as u64)+1)) as u32));
        //println!("num_original cambia a {}, reverse_i es {}, elemento actual: {}\n", num_original, i as u64, num_vect[(i as usize)]);
    }

    num_vect.clear();

    // Conversion of num_original to base2
    // via a while loop with % and / (u64)
    while num_original != 0{
        num_vect.insert(0, num_original % base2);
        num_original /= base2;
    }

    // Makes sure the data is correct 2, with base2.
    for data in 0..num_vect.len(){
        if (num_vect[(data as usize)] as u64) > base2 {
            println!("The digit {} is greater than the base.", data);
            ()
        }
    };

    num_vect
}

// base1_case1 is not by default 10, base1_case1 can vary
fn base_converter_case2(num1 : u64, base1 : u64, base2 : u64) -> Vec<u64> {
    // Edge cases of base_conversion.
    // Insert them if the program doesn't cover them already
    if base1 < 2{
        println!("Base1 cannot be lower than 2.");
        std::process::exit(0);
    } else if base2 < 2{
        println!("Base2 cannot be lower than 2.");
        std::process::exit(0);
    };

    // Vector and unsigned int definitions.
    let mut num_vect : Vec<u64> = Vec::new();
    let mut num_original : u64 = num1;

    // Obtain the number as a vector with u64 integers.
    while num_original != 0{
        //println!("{} is num_original",num_original);
        if num_original == 0{
            break;
        }
        num_vect.insert(0,(num_original % base1).try_into().unwrap());
        num_original /= base1;
    };

    // Define a "constant" so it has that value stored.
    let LNV : u64 = (num_vect.len() as u64); 
                                      

    // Makes sure the data is correct.
    for data in 0..LNV{
        if (num_vect[(data as usize)] as u64) > base1{
            println!("The digit {} is greater than the base.", data)
        }
    };

    // Obtains the real value of the num1.
    for i in 0..LNV{
        num_original += (num_vect[(i as usize)] * base1.pow((LNV - ((i as u64)+1)) as u32));
        //println!("num_original cambia a {}, reverse_i es {}, elemento actual: {}\n", num_original, i as u64, num_vect[(i as usize)]);
    }

    num_vect.clear();

    // Conversion of num_original to base2
    // via a while loop with % and / (u64)
    while num_original != 0{
        num_vect.insert(0, num_original % base2);
        num_original /= base2;
    }

    // Makes sure the data is correct 2, with base2.
    for data in 0..num_vect.len(){
        if (num_vect[(data as usize)] as u64) > base2 {
            println!("The digit {} is greater than the base.", data)
        }
    };

    num_vect
}

// The test part of the program,
// with constant num, base1 and base2.
fn main_test() {
    const num_test : u64 = 100;
    const base1_test : u64 = 10;
    const base2_test : u64 = 2;

    // Case 1: Default base is 10.
    let mut result : Vec<u64> = base_converter_case1(num_test, base2_test);

    if result.len() == 0{
        println!("Result cannot have length zero.");
        std::process::exit(1)
    };

    println!("Number ({}), base 10, is this number in base {}:",num_test, base2_test);
    for data in 0..result.len(){
        print!("{} ",result[(data as usize)]);
    };
    println!("\n");
    
    // Case 2: No default base.
    result = base_converter_case2(num_test, base1_test, base2_test);
    println!("Number ({}), base {}, is this number in base {}:",num_test, base1_test, base2_test);

    for data in 0..result.len(){
        print!("{} ",result[(data as usize)]);
    };

    println!("\n");
}

fn main(){
    println!("Would you like to use the test?\n");
    while 1 == 1 {
        print!("(Y/N) > ");
        let _ = io::stdout().flush();
        let mut input_2: String = String::new();
        let _ = io::stdin().read_line(&mut input_2);
        input_2 = input_2.to_ascii_uppercase();
        if input_2 != ""{
            let input : &str = input_2.as_str().trim();
            if input == "Y"{
                println!("Here it is:");
                main_test();
                println!("Next, your conversion:\n");
                break;
            } else if input == "N"{
                println!("Then, let's continue: \n");
                break;
            } else {
                println!("That option does not exist.\n");
                continue;
            }
        }
    }


    // Read arguments as a vector.
    
    let args: Vec<String> = env::args().collect();

    // Check if there are enough arguments in the program.
    let arg = match args.get(3) {
        Some(val) => val,
        None => {
            println!("Not enough arguments provided!");
            std::process::exit(0);
        }
    };

    // Check if arguments can be parsed into u64.
    match arg.parse::<u64>() {
        Ok(val) => val,
        Err(e) => {
            println!("Unable to parse number from argument: {}", e);
            std::process::exit(0);
        }
    };

    // Define the constants
    let num : u64 = args[1].parse().unwrap();
    let base1 : u64 = args[2].parse().unwrap();
    let base2 : u64 = args[3].parse().unwrap();

    let result : Vec<u64> = base_converter_case2(num, base1, base2);

    // A conversion must result in a length one or higher vector.
    if result.len() == 0{
        println!("Result cannot have length zero.");
        std::process::exit(1)
    };

    println!("Number ({}), base 10, is this number in base {}:",num, base2);
    for data in 0..result.len(){
        print!("{} ",result[(data as usize)]);
    };

    println!("\n");
}
