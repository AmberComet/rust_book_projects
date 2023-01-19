use std::io;
use question::{Answer,Question};

fn main() {


    loop{
    //prompt the user and get the first num
   println!("enter the first number to factor");
    let x = input();

    //prompt the user and get the second num
    println!("enter the second number to factor");
    let y = input();
  

    println!("The factors of {x} are:");
    print_facts(factor(y));
    println!("The factors of {y} are:");
    print_facts(factor(x));
    let same:Vec<u32> = facts_compair(factor(x),factor(y));
    
    println!("The shared factors are:");
    print_facts(same);

    let answer = Question::new("Continue?")
        .default(Answer::YES)
        .show_defaults()
        .confirm();

    if answer == Answer::YES{
        continue
    }else{
        break
        }
    }
    println!("end of program");
}

//function to take user input
fn input()-> u32{
    
    //new empty string 
    let mut x=String::new();

    //read next line from standard in 
    io::stdin()
         .read_line(&mut x)
         .expect("Failed to read line");
 
     //parse the string to a u32
     let x: u32 = match x.trim().parse() {
         Ok(num) => num,
         Err(_) => todo!(),
     };
     
    return x;
}

//factorize a number 
fn factor(x: u32) -> Vec<u32>{
    //create a new vector empty vector (similar to ArrayList in java)
    let mut fact: Vec<u32> = Vec::new();

    // create a control var 
    let mut i=1;

    //factorize the number 
    //TODO optamize this algortim to only to i<=x/2 then add i and x/i
    while i<=x{
        if x%i==0{
                fact.push(i);
            }
       i=i+1;
    }
    return fact;
}

//print out the factors 

fn print_facts(facs:Vec<u32>){
    let check = facs[facs.len()-1];
print!("[");
    for i in facs {
        if i!=check{
        print!("{i},");
        }else{
            print!("{i}")
        }
    }
    print!("]\n")
}

fn facts_compair(facs_x:Vec<u32>,facs_y:Vec<u32>) ->Vec<u32>{
    let mut union:Vec<u32> = Vec::new();

    for i in &facs_x{
        for j in &facs_y{
            if i==j{
                union.push(*i);
            }
        }
    }
   return union;
}