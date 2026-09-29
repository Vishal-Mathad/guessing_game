use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number !");

    // thread_rng() -> This is a generator that has a function gen_range() -> this generates a random number between 1 and 100 and 1 and 100 are included
    // Exclusive -> (0..100) & Inclusive -> (0..=100)
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Secret Number is : {}", secret_number);

    // This is a mutuable variable -> because we have used mut
    // In rust all the variables are immutable by default and we cannot change there values once we have initialized them so we create mutuable variables if we need them.

    // This is a loop now that means we can now guess the number infinite times as the terminal will not break/crash.
    // But this is also not correct as it should break after we have won so use break statement according to the challenge.
    loop {
        let mut guess = String::new();

        println!("Please input your number.");

        // io-> input/output
        // stdin -> standard input -> taking input and we have read_line : Its job is to read line by line and append the reference given to it.
        // Appending is the key word : It does not overwrite it appends only
        // References : In rust if you want to manipulate variables you provide there mutuable references and that is a way you can do changes inside that variables.
        // read_line gives result as an output which is a enum and we dont handle ok, but if any error happens we expect that to happen and if so we print that message and we crash there.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // you can have empty {} and with a comma can mention the value that must be provided inside that bracket.
        // We mainly use this technique when we have an expression and the value is going to update so we use this there
        println!("You guessed ! : {}", guess);

        // We cannot compare string with integer so we will have to parse that into integer.
        // We have used the same variable name guess -> This is called shadowing in which we use the same variable name and this reference will be used now onwards the previous reference is removed.
        // Why did we trim it -> because when we gave an input and pressed enter the input did not just have the value we gave it also had the next line \n inside it so we need to trim so that we are able to parse the string.
        let guess: u32 = guess.trim().parse().expect("Failed to parse !");

        // cmp -> comparsion operator hai which compares the two values
        // match is basically switch case kind of -> It strictly tells to handle all the possible scenarios. for eg we are comparing two numbers -> possible sceanrios are either it is greater or smaller and it can be equal so we need to handle all 3 scenarios.
        // cmp -> returns an Ordering so we have to handle that
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You Won !!");
                break;
            }
            Ordering::Greater => println!("Too Big"),
        }
    }
}
