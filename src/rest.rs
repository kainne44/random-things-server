
use crate::app::{Input, DiceParams, NumParams, random};


use poem_openapi::{
    payload::Json,
    ApiResponse, Object, OpenApi,
};

#[derive(Object)]
pub struct RandObj {
    req_type: String,
    qty: Option<usize>,
    size: Option<usize>,
    sides: Option<usize>,
}

#[derive(ApiResponse)]
pub enum RandResponse {
    // returns when random value is generated
    #[oai(status = 200)]
    Ok(Json<Vec<String>>),

    // returns when random value could not be generated
    #[oai(status = 501)]
    NotImplemented,
}

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/rand", method = "post")]
    pub async fn index(&self, request: Json<RandObj>) ->  RandResponse {
        // TODO: need to parse request and add app logic here from app.rs 
        let rand_type = &request.req_type; 
        let input;
        let output_vec: Vec<String>;
        match rand_type.as_str() {
            "dice" => {
                input = Input::Dice(DiceParams {qty: request.qty.unwrap(), sides: request.sides.unwrap()});
            },
            "number" => {
                input = Input::Number(NumParams {size: request.size.unwrap()});},
            "password" => {
                input = Input::Password(NumParams {size: request.size.unwrap()});},
            _ => return RandResponse::NotImplemented 
        }
        output_vec = random(input).unwrap(); 
        let output = Json(output_vec);
        RandResponse::Ok(output)
    }
    // TODO: need a function that handles the get request at /rand
    // it is likely that this would be what calls the frontend page
}
