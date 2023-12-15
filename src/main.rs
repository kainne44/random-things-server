/* This program is intended to be a random number generator
 * The idea is to be able to generate random numbers, passwords,
 * and dice rolls based on the request from a client*/

use actix_web::{get, post, web::{self, Json}, App, HttpResponse, HttpServer, Responder, HttpRequest};
use rand::Rng;
use serde::Deserialize;

#[allow(dead_code)]
struct NumParams {
    size: usize,
}

#[allow(dead_code)]
struct DiceParams {
    qty: usize,
    sides: usize,
}

#[allow(dead_code)]
enum Input {
    Number(NumParams),
    Dice(DiceParams),
    Password(NumParams),
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[post("/echo")]
async fn echo(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(info.username.clone())
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: Web Server Functionality
    // Need to add new routes
    HttpServer::new(|| {
        App::new().service(hello).service(echo).route("/hey", web::get().to(manual_hello))
    }).bind(("127.0.0.1", 8080))?.run().await


    
    //TODO: Parse Requests and conform input to what the BL can accept
    // All of the logic has to go in the implementation function for the
    // http request

    // Business Logic - Generating random things

    // let input = Input::Number(NumParams { size: 6 });
    // let input = Input::Dice(DiceParams { qty: 2, sides: 20 });
    // let input = Input::Password(NumParams { size: 12 });
    //
    // let str_output: Vec<String>;
    // match input {
    //     Input::Number(input) => {
    //         str_output = vec![rng_by_size(input.size)];
    //     }
    //     Input::Dice(input) => {
    //         str_output = dice_roll(input.qty, input.sides);
    //     }
    //     Input::Password(input) => {
    //         str_output = vec![rand_pwd(input.size)];
    //     }
    // }
    // dbg!(str_output);
}

// *FUNCTION* returns a random number num_len digits long
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    return password;
}
