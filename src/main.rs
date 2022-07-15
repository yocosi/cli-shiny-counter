use anyhow::{Context, Ok, Result};
use std::fs;
use std::fs::File;
use std::io;
use std::io::*;
use std::path::Path;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// Get the input of the user and return the path
fn user_input_for_counter_name() -> String {
    let mut counter_name = String::new();
    io::stdin()
        .read_line(&mut counter_name)
        .expect("error: unable to read the name for the new counter.");
    if counter_name == "q\n" || counter_name == "Q\n" {
        return counter_name.replace("\n", "");
    } else {
        let counter_path = format!("src/counters/{}{}", counter_name, ".txt");
        let full_counter_path = counter_path.replace("\n", "");
        return full_counter_path;
    }
}

// Handle the counter: Open the file needed, parse the content of the file to a number, clear the terminal and allow the user to press the necessary keys to modify the file
fn counter_handling(full_counter_path: String) -> Result<()> {
    // File::options() used here to be able to write in the file with the .write(true) method
    let mut current_counter = File::options()
        .read(true)
        .write(true)
        .open(&full_counter_path)
        .with_context(|| format!("error: could not open the file `{}`", &full_counter_path))?;

    let mut buffer = String::new();
    let mut counter: u64 = current_counter
        .read_to_string(&mut buffer)
        .unwrap()
        .to_string()
        .parse()
        .with_context(|| {
            format!(
                "error: unable to parse the String of the counter of the file `{}` into a number",
                &full_counter_path
            )
        })?;

    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();

    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    write!(stdout, r#"{}{}Counter ready. Press 'up-arrow' to increment and 'down-arrow' to decrement the counter. Press (q/Q) to quit.""#, termion::cursor::Goto(1, 1), termion::clear::All)
            .unwrap();

    stdout.flush().unwrap();

    for key in stdin().keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match key.unwrap() {
            Key::Up => {
                println!("{}", counter);
                fs::write(&full_counter_path, &counter.to_string()).with_context(|| {
                    format!(
                        "error: could not increment the counter of the file `{}`",
                        &full_counter_path
                    )
                })?;
                counter += 1;
            }
            Key::Down => {
                println!("{}", counter);
                fs::write(&full_counter_path, &counter.to_string()).with_context(|| {
                    format!(
                        "error: could not decrement the counter of the file `{}`",
                        &full_counter_path
                    )
                })?;
                counter -= 1;
            }
            Key::Char('q') | Key::Char('Q') => break,
            _ => (),
        }
        stdout.flush().unwrap();
    }
    Ok(())
}

// Create the txt file and write 0 in it to make it become a new counter.
// And this function takes counters_list as a parameter to push the new counter in it to be able to use it for the listing of all the counter in the show_counters() function
fn add_counter(counters_list: &mut Vec<String>) -> Result<()> {
    loop {
        println!("Enter a counter name (or `q` to go back to the main menu): ");
        let full_counter_path = user_input_for_counter_name();
        let mut file_name = full_counter_path.replace("src/counters/", "");
        file_name = file_name.replace(".txt", "");

        if full_counter_path == "q" || full_counter_path == "Q" {
            break;
        } else if Path::new(&full_counter_path).exists() {
            println!("error: The counter already exist!");
        } else {
            counters_list.push(file_name);
            let mut new_file = File::create(&full_counter_path).with_context(|| {
                format!("error: could not create the file `{}`", &full_counter_path)
            })?;
            write!(&mut new_file, "0").unwrap();
            println!("The counter has succesfully been created");
            break;
        }
    }
    Ok(())
}

// This function select the counter that the user wants to use and call the counter_handling() function to handle all the counters mechanics
fn select_counter() -> Result<()> {
    loop {
        println!("Enter the name of the counter that you want to select (or `q` to go back to the main menu): ");
        let full_counter_path = user_input_for_counter_name();

        if full_counter_path == "q" || full_counter_path == "Q" {
            break;
        } else if !Path::new(&full_counter_path).exists() {
            println!("error: The counter does not exist!");
        } else {
            counter_handling(full_counter_path)?;
        }
    }
    Ok(())
}

// This function takes counters_list as a parameter which contain all the counters available and loop through them to print their names and the counter number
fn show_counters(counters_list: &mut Vec<String>) -> Result<()> {
    for counter in counters_list {
        let file_path = format!("src/counters/{}{}", counter, ".txt");
        let mut file = File::open(&file_path)
            .with_context(|| format!("error: could not open the file `{}`", &file_path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .with_context(|| format!("error: unable to read the file `{}`", &file_path))?;
        println!("{} {}", counter, contents);
    }
    Ok(())
}

// This function takes an existing counter and reset it to 0
fn reset_counter() -> Result<()> {
    loop {
        println!("Enter a counter name (or `q` to go back to the main menu): ");
        let full_counter_path = user_input_for_counter_name();

        if full_counter_path == "q" || full_counter_path == "Q" {
            break;
        } else if !Path::new(&full_counter_path).exists() {
            println!("error: The counter doesn't exist!");
        } else {
            let mut file = File::create(&full_counter_path).with_context(|| {
                format!("error: could not open the file `{}`", &full_counter_path)
            })?;
            write!(&mut file, "0").unwrap();
            println!("The counter has succesfully been reseted");
            break;
        }
    }
    Ok(())
}

// This function takes an existing counter and delete it
fn delete_counter() -> Result<()> {
    loop {
        println!("Enter a counter name (or `q` to go back to the main menu): ");
        let full_counter_path = user_input_for_counter_name();

        if full_counter_path == "q" || full_counter_path == "Q" {
            break;
        } else if !Path::new(&full_counter_path).exists() {
            println!("error: The counter doesn't exist!");
        } else {
            fs::remove_file(&full_counter_path).with_context(|| {
                format!("error: could not remove the file `{}`", &full_counter_path)
            })?;
            println!("The counter has succesfully been deleted");
            break;
        }
    }
    Ok(())
}

fn main() {
    let mut counters_list: Vec<String> = vec![];
    loop {
        println!(
            "
            SELECT ONE OF THIS OPTION:\n
            1) Add a counter.
            2) Select a specific counter.
            3) See all the counters.
            4) Reset a specific counter.
            5) Delete a specific counter.
            q) Quit."
        );
        let mut user_input = String::with_capacity(1);
        io::stdin()
            .read_line(&mut user_input)
            .expect("error: unable to read the user input");

        // Switch like in rust
        match user_input.as_str().trim() {
            "1" => add_counter(&mut counters_list),
            "2" => select_counter(),
            "3" => show_counters(&mut counters_list),
            "4" => reset_counter(),
            "5" => delete_counter(),
            "q" | "Q" => break,
            &_ => Ok(println!("Bad selection. You must choose between 1 and 6.")),
        }
        .ok();
    }
}
