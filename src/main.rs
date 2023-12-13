/* This program is intended to be a random number generator
 * The idea is to be able to generate random numbers, passwords,
 * and dice rolls based on the request from a client*/

use color_eyre::eyre::Result;
use rand::Rng;

struct NumParams {
    size: usize,
}

struct DiceParams {
    qty: usize,
    sides: usize,
}

enum Input {
    Number(NumParams),
    Dice(DiceParams),
    Password(NumParams),
}

fn main() {
    // TODO: Add web server functionality
    // TODO: Allow user to pass the selections as inputs
    //
    // For example, a user may select "random number" and pick 16 to generate
    // a 16 digit random number.
    // OR they may select "password" and pick 12 to generate a 12 letter random pw
    // OR they may select "dice", passing the qty of dice and sides

    // let input = Input::Number(NumParams { size: 6 });
    let input = Input::Dice(DiceParams { qty: 2, sides: 20 });
    // let input = Input::Password(NumParams { size: 12 });

    let mut str_output: Vec<String> = Vec::new();
    match input {
        Input::Number(input) => {
            str_output = vec!(rng_by_size(input.size));
        }
        Input::Dice(input) => {
            str_output = dice_roll(input.qty, input.sides);
        }
        Input::Password(input) => {
            str_output = vec!(rand_pwd(input.size));
        }
    }
    dbg!(str_output);
}

// *FUNCTION* returns a random number num_len digits long
fn rng_by_size(num_len: usize) -> String {
    let mut rng = rand::thread_rng();

    // you need to have a vector to concat to
    let mut num_str_vec: Vec<String> = Vec::new();

    // loop thru the num of values and concatenate a random number
    for _n in 0..num_len {
        let random_num: usize = rng.gen_range(0..10);
        num_str_vec.push(random_num.to_string());
    }
    let random_num: String = num_str_vec.join("").to_string();
    return random_num;
}

// *FUNCTION* returns a vector of values from 1 to n, where n is sides of die
fn dice_roll(qty: usize, sides: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut dice_vec: Vec<String> = Vec::new();
    for _die in 1..=qty {
        let roll: usize = rng.gen_range(1..=sides);
        dice_vec.push(roll.to_string());
    }
    dice_vec
}

// *FUNCTION* returns a string of random chars of n length
// TODO: Finish this function
fn rand_pwd(size: usize) -> String {
    let pwd = "pwd".to_string();
    return pwd;
}
