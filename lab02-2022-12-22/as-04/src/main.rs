
use std::io;
fn main() {
    println!("Enter number:");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("invalid input");
    let mut total = 1;
    print!("{} = ",input);
    while total != input{
        if total * 2 <= input {
            if  input % 2 == 0 {
                total = total*2;
                print!("2");
                if total == input {break;}
                else {print!("*");}
        }}
        if total * 3 <= input {
            if  input % 3 == 0 {
                total = total*3;
                print!("3");
                if total == input {break;}
                else {print!("*");}
        }}
        if total * 5 <= input {
            if  input % 5 == 0 {
                total = total*5;
                print!("5");
                if total == input {break;}
                else {print!("*");}
        }}
        if total * 7 <= input {
            if  input % 7 == 0 {
                total = total*7;
                print!("7");
                if total == input {break;}
                else {print!("*");}
        }} 
        if total * 11 <= input {
            if  input % 11 == 0 {
                total = total*11;
                print!("11");
                if total == input {break;}
                else {print!("*");}
        }}  
}
}

