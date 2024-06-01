use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

/**
* Represant the claims inside the JWT
* user : the address of the user
* exp : the expiration date of the claim WARNING : keep the exp name otherwise there is issue with the jsonwebToken crate
*/
#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub address: String,
    pub exp: usize,
}

/**
* Create a new claims given an address, and make it expires in 1 hour
*/
impl Claims {
    pub fn new(address: String) -> Self {
        Self {
            address: address.to_lowercase(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        }
    }
}
