use actix_web::{guard::GuardContext, http::header};
use jsonwebtoken::{
    decode,
    DecodingKey,
    Validation
};

pub fn authorization_guard(ctx: &GuardContext<'_>) -> bool {
    // retrieve authorization token from header
    // validate
    let secret = b"I WOULD EXIST IN THE CONFIG NOT HERE";
    let header = ctx.head().headers().get(header::AUTHORIZATION);
    if header.is_none() {
        return false
    }
    let header_value = header.unwrap();
    println!("{:?}", header_value);
    true
}
