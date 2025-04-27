use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NFTError {
    AdminAlreadyExists = 1,
    TokenExists = 2,
    NotAuthorized = 3,
    TokenDoesNotExist = 4,
    InvalidTokenOwner = 5,
}