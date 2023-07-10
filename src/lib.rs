#![no_std]

use soroban_sdk::{
    contractimpl, contracttype, map, vec, Address, Bytes, BytesN, Env, Map, String, Symbol, Vec,
};

const IS_BOOLEAN: bool = true;
const SIGNED_INTEGER: i32 = -10;
const UNSIGNED_INTEGER: u32 = 10;
const EXAMPLE: Symbol = Symbol::short("EXAMPLE");

#[contracttype]
pub enum ExampleEnum {
    A,
    B,
    C(u32),
}

pub enum IntegerEnum {
    A = 0,
    B = 1,
    C = 2,
}

pub struct State {
    pub owner: Symbol,
    pub amt: u32,
}

const STATE: State = State {
    owner: Symbol::short("BOB"),
    amt: 44,
};

fn get_owner() -> Symbol {
    STATE.owner
}

fn get_amt() -> u32 {
    STATE.amt
}

pub struct Tuple(u32, Symbol);

const TUPLE_EXAMPLE: Tuple = Tuple(777, Symbol::short("TUPLE_EX"));

fn get_tuple_slot_0() -> u32 {
    TUPLE_EXAMPLE.0
}

fn get_tuple_slot_1() -> Symbol {
    TUPLE_EXAMPLE.1
}

fn set_a(e: &Env, a: Symbol) {
    let a_symbol = vec![&e, a];
    e.storage().set(&ExampleEnum::A, &a_symbol);
}

fn get_a(e: Env) -> Vec<Symbol> {
    e.storage()
        .get(&ExampleEnum::A)
        .expect("not initialized")
        .unwrap()
}

fn set_b(e: &Env, b: Address) {
    e.storage().set(&ExampleEnum::B, &b);
}

fn get_b(e: Env) -> Address {
    e.storage()
        .get(&ExampleEnum::B)
        .expect("not initialized")
        .unwrap()
}

fn set_c(e: &Env, c: u32) {
    e.storage().set(&ExampleEnum::C(0), &c);
}

fn get_c(e: Env) -> u32 {
    e.storage()
        .get(&ExampleEnum::C(0))
        .expect("not initialized")
        .unwrap()
}

fn get_a2() -> IntegerEnum {
    IntegerEnum::A
}

fn get_b2() -> IntegerEnum {
    IntegerEnum::B
}

fn get_c2() -> IntegerEnum {
    IntegerEnum::C
}

pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn initialize(contract_owner: Address) {
        let env = Env::default();
        env.storage().set(&ExampleEnum::B, &contract_owner);
    }

    pub fn set_a(a: Symbol) {
        let e = Env::default();
        set_a(&e, a);
    }

    pub fn get_a(e: Env) -> Vec<Symbol> {
        get_a(e)
    }

    pub fn set_b(b: Address) {
        let e = Env::default();
        set_b(&e, b);
    }

    pub fn get_b(e: Env) -> Address {
        get_b(e)
    }

    pub fn set_c(c: u32) {
        let e = Env::default();
        set_c(&e, c);
    }

    pub fn get_c(e: Env) -> u32 {
        get_c(e)
    }

    pub fn set_a_b_c(a: Symbol, b: Address, c: u32) {
        let e = Env::default();
        set_a(&e, a);
        set_b(&e, b);
        set_c(&e, c);
    }

    pub fn get_a_b_c(e: Env) -> (Vec<Symbol>, Address, u32) {
        (get_a(e.clone()), get_b(e.clone()), get_c(e))
    }

    pub fn get_a2() -> u32 {
        get_a2() as u32
    }

    pub fn get_b2() -> u32 {
        get_b2() as u32
    }

    pub fn get_c2() -> u32 {
        get_c2() as u32
    }

    pub fn get_owner_from_state() -> Symbol {
        get_owner()
    }

    pub fn get_amt_from_state() -> u32 {
        get_amt()
    }

    pub fn get_tuple_slot_0() -> u32 {
        get_tuple_slot_0()
    }

    pub fn get_tuple_slot_1() -> Symbol {
        get_tuple_slot_1()
    }

    pub fn get_boolean() -> bool {
        IS_BOOLEAN
    }
    pub fn get_integer() -> i32 {
        SIGNED_INTEGER
    }
    pub fn get_unsigned_integer() -> u32 {
        UNSIGNED_INTEGER
    }
    pub fn get_symbol() -> Symbol {
        EXAMPLE
    }
    pub fn get_long_symbol() -> Symbol {
        let env = Env::default();
        Symbol::new(&env, "THIS_IS_AN_EXCEEDINGLY_LONG_SYMBOL")
    }
    pub fn get_string() -> String {
        let env = Env::default();
        let msg = "a message";
        let new_string = String::from_slice(&env, msg);
        new_string
    }
    pub fn get_bytes() -> Bytes {
        let env = Env::default();
        let msg = "a message";
        let new_bytes = Bytes::from_slice(&env, msg.as_bytes());
        new_bytes
    }
    pub fn get_bytesn() -> BytesN<32> {
        let env = Env::default();
        let new_bytesn = BytesN::from_array(&env, &[1; 32]);
        new_bytesn
    }
    pub fn get_address() -> Address {
        let env = Env::default();
        let current_address = env.current_contract_address();
        current_address
    }
    pub fn get_vec() -> Vec<u32> {
        let env = Env::default();
        let new_vec = vec![&env, 1, 2, 3];
        new_vec
    }
    pub fn get_vec_symbol() -> Vec<Symbol> {
        let env = Env::default();
        let hello = Symbol::short("hello");
        let and = Symbol::short("and");
        let bob = Symbol::short("bob");
        let alice = Symbol::short("alice");
        let new_vec = vec![&env, hello, and, bob, alice];
        new_vec
    }
    pub fn get_map() -> Map<u32, u32> {
        let env = Env::default();
        let new_map = map![&env, (1, 10), (2, 20)];
        new_map
    }
    pub fn get_timestamp() -> u64 {
        let env = Env::default();
        env.ledger().timestamp()
    }
    pub fn get_sequence() -> u64 {
        let env = Env::default();
        env.ledger().sequence().into()
    }
}
