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

enum RandomType {
    Number(NumParams),
    Dice(DiceParams),
    Password(NumParams),
}

fn main() {
    // TODO: Add web server functionality w/ axum
    // TODO: Allow user to pass the selections as inputs
    //
    // For example, a user may select "random number" and pick 16 to generate
    // a 16 digit random number.
    // OR they may select "password" and pick 12 to generate a 12 letter random pw
    // OR they may select "dice", passing the qty of dice and sides

    let input_number = RandomType::Number(NumParams { size: 6 });
    let input_dice = RandomType::Dice(DiceParams { qty: 2, sides: 6 });
    let input_password = RandomType::Password(NumParams { size: 12 });

    // size of concatenated random number -- TODO: input from user
    let num_len: usize = 10;
    let number = rng_by_size(num_len);
    dbg!(number);

    // need random dice roll, lets you choose qty of sides and qty of dice

    // need random password, lets you choose length of password
}

// *FUNCTION* returns a random number num_len digits long
fn rng_by_size(num_len: usize) -> usize {
    let mut rng = rand::thread_rng();

    // you need to have a vector to concat to
    let mut num_str_vec: Vec<String> = Vec::new();

    // loop thru the num of values and concatenate a random number
    for _n in 0..num_len {
        let random_num: usize = rng.gen_range(0..10);
        num_str_vec.push(random_num.to_string());
    }
    let random_num: usize = num_str_vec.join("").parse().unwrap();
    return random_num;
}
