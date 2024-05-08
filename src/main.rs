use std::io;

fn main() {
    println!("Hello, world! This is a simple calculator!");
    inputs();

    loop {
        let mut operation = String::new();

        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line!");

        operation = operation.trim().to_string();

        match operation.as_str() {
            "quit" => break,
            "operations" => {
                inputs();
                continue;
            },
            "add" => add(input_handling()),
            "subtract" => subtract(input_handling()),
            "multiply" => multiply(input_handling()),
            "divide" => divide(input_handling()),
            _ => println!("Unclear input. Try again!"),
        }

        println!("Start all over again!");
    }
}

fn add((x, y): (f64, f64)) {
    println!("{:?} + {:?} = {:?}", x, y, x + y);
}

fn subtract((x, y): (f64, f64)) {
    println!("{:?} - {:?} = {:?}", x, y, x - y);
}

fn multiply((x, y): (f64, f64)) {
    println!("{:?} * {:?} = {:?}", x, y, x * y);
}

fn divide((x, y): (f64, f64)) {
    println!("{:?} / {:?} = {:?}", x, y, x / y);
}

fn inputs() {
    println!("Here are the operations you can do:
        add: addition
        subtract: subtraction
        multiply: multiplication
        divide: division
        quit: breaks the loop, ends the program
        operations: see the list of operations again
        any other input: will not work
        ------------------------------
        input your operation now");
}

fn input_handling() -> (f64, f64) {
    println!("What are the two numbers?
    Please type the first number, then press enter. Then type the second number and press enter
    That is the ONLY input method that will be accepted.");

    let mut input1 = String::new();

    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line!");

    let index0: f64 = input1
        .trim()
        .parse()
        .expect("You did not enter a number");

    let mut input2 = String::new();

    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line!");

    let index1: f64 = input2
        .trim()
        .parse()
        .expect("You did not enter a number");

    let tup: (f64, f64) = (index0, index1);
    tup
}

/* Notes:
1: idk if this is just vscode or how rust actually works, but apparently I don't need \n to create a new line, and pressing enter and tab in string literals corresponds
to actual enters and tabs in the string literal. also pressing enter automatically adds one tab, so enter + tab is enter + 2 tab
3: at first I wanted to have the ability to use your result to calculate more, but I didn't know how do to that. I'll add that functionality in the future
4: don't like how i have to type add/subtract/multiply etc everytime a calculation is done, but i think that's just a quirk of working with a terminal, 
even if i make it easier with a/s/m/d, it's still a quirk
5: error handling when a number isn't entered is suboptimal. if it's not a number, it crashes on purpose. but if i didn't make it crash, i don't know how to let the
user retry to enter a number since input_handling() is not part of the loop in main(). would have to refactor code to make this possible
6: not sure if the method of entering numbers in input_handling() is the best way, but i thought it was a) the simplest for me to handle, and b) one of the easiest for a user
to understand
7: in the future, i will add the functionality of doing an operation with more than two numbers at once, perhaps with more than one operation too. 
 */
