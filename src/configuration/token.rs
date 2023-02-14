use crate::utils::StrongTypedef;

pub struct TokenTag;
pub type Token = StrongTypedef<String, TokenTag>;

impl Token {

}
