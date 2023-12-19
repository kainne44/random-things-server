use color_eyre::eyre::Report;
use rand::Rng;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct NumParams {
    pub size: usize,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct DiceParams {
    pub qty: usize,
    pub sides: usize,
}

#[derive(Deserialize)]
pub enum Input {
    Number(NumParams),
    Dice(DiceParams),
    Password(NumParams),
}

pub fn random(input: Input) -> Result<Vec<String>, Report> {
    let str_output: Vec<String>;
    match input {
        Input::Number(input) => {
            str_output = vec![rng_by_size(input.size)];
        }
        Input::Dice(input) => {
            str_output = dice_roll(input.qty, input.sides);
        }
        Input::Password(input) => {
            str_output = vec![rand_pwd(input.size)];
        }
    }
    Ok(str_output)
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
// I got this algorithm from the "Rust Cookbook"
fn rand_pwd(size: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789*&^%$#@!~";
    let mut rng = rand::thread_rng();
    let password: String = (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}
