use {
    asn1::{
        der::{from_slice, to_vec},
        types::Enumerable,
    },
    serde::{Deserialize, Serialize},
};

fn main() {
    env_logger::init();
    let input = Alpha::A(Bravo::B);

    assert_eq!(input, from_slice(&to_vec(&input).unwrap()).unwrap())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Alpha {
    A(Bravo),
    B(Bravo),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Bravo {
    A,
    B,
}
