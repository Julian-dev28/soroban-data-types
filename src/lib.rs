#![no_std]

use soroban_sdk::{
    contractimpl, contracttype, map, vec, Address, Bytes, BytesN, Env, Map, String, Symbol, Vec,
};

const IS_BOOLEAN: bool = true;
const SIGNED_INTEGER: i32 = -10;
const UNSIGNED_INTEGER: u32 = 10;
const EXAMPLE: Symbol = Symbol::short("EXAMPLE");

// Enums
#[contracttype]
pub enum ExampleEnum {
    A,
    B,
    C(u32),
    D,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StatusEnum {
    Open = 0,
    Closed = 1,
    Pending = 2,
}

// Structs
pub struct NamedFieldStruct {
    pub name: Symbol,
    pub amt: u32,
}
pub struct UnnamedFieldStruct(Symbol, u32);

const STRUCT_NAMED_FIELD: NamedFieldStruct = NamedFieldStruct {
    name: Symbol::short("STRUCT"),
    amt: 999,
};

const STRUCT_UNNAMED_FIELD: UnnamedFieldStruct = UnnamedFieldStruct(Symbol::short("STRUCT2"), 777);

pub struct DataTypes;

#[contractimpl]
impl DataTypes {
    pub fn initialize(contract_owner: Address, address_random: Address, c_val: u32, d_val: Symbol) {
        let env = Env::default();
        env.storage().set(&ExampleEnum::A, &contract_owner);
        env.storage().set(&ExampleEnum::B, &address_random);
        env.storage().set(&ExampleEnum::C(0), &c_val);
        env.storage().set(&ExampleEnum::D, &d_val);
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
        let msg = "hello, world";
        let new_string = String::from_slice(&env, msg);
        new_string
    }
    pub fn get_bytes() -> Bytes {
        let env = Env::default();
        let msg: &str = "hello, world";
        let new_bytes: Bytes = Bytes::from_slice(&env, msg.as_bytes());
        new_bytes
    }
    pub fn get_bytesn() -> BytesN<32> {
        let env: Env = Env::default();
        let msg: &str = "hello, world";

        // Create an array of 32 zeroed bytes
        let mut msg_to_array: [u8; 32] = [0; 32];

        // Copy the bytes of your message into the array
        msg_to_array[..msg.len()].copy_from_slice(msg.as_bytes());

        let new_bytesn: BytesN<32> = BytesN::from_array(&env, &msg_to_array);
        new_bytesn
    }

    pub fn get_vec_int() -> Vec<u32> {
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
    pub fn get_vec_address() -> Vec<Address> {
        let env = Env::default();
        let address1 = Self::get_a_address(env.clone());
        let address2 = Self::get_b_address(env.clone());
        let new_vec = vec![&env, address1, address2];
        new_vec
    }

    pub fn get_named_field_name() -> Symbol {
        STRUCT_NAMED_FIELD.name
    }

    pub fn get_named_field_amt() -> u32 {
        STRUCT_NAMED_FIELD.amt
    }

    pub fn get_unnamed_field_slot_0() -> Symbol {
        STRUCT_UNNAMED_FIELD.0
    }

    pub fn get_unnamed_field_slot_1() -> u32 {
        STRUCT_UNNAMED_FIELD.1
    }

    pub fn get_open_status() -> u32 {
        StatusEnum::Open as u32
    }

    pub fn get_close_status() -> u32 {
        StatusEnum::Closed as u32
    }

    pub fn get_pending_status() -> u32 {
        StatusEnum::Pending as u32
    }

    pub fn get_map(key1: Address, value1: u32, key2: Address, value2: u32) -> Map<Address, u32> {
        let env = Env::default();
        let new_map: Map<Address, u32> = map![&env, (key1, value1), (key2, value2)];
        new_map
    }

    pub fn get_deadline() -> u64 {
        let env = Env::default();
        let deadline: u64 = env.storage().get(&ExampleEnum::C(100)).unwrap().unwrap();
        deadline
    }

    pub fn get_a_b_c_d(c_slot: u32) -> (Address, Address, u32, Symbol) {
        let env = Env::default();
        let a = Self::get_a_address(env.clone());
        let b = Self::get_b_address(env.clone());
        let c = Self::get_c_u32(env.clone(), c_slot);
        let d = Self::get_d_symbol(env.clone());
        (a, b, c, d)
    }

    pub fn get_a_address(e: Env) -> Address {
        e.storage()
            .get(&ExampleEnum::A)
            .expect("not initialized")
            .unwrap()
    }

    pub fn get_b_address(e: Env) -> Address {
        e.storage()
            .get(&ExampleEnum::B)
            .expect("not initialized")
            .unwrap()
    }

    pub fn get_c_u32(e: Env, slot: u32) -> u32 {
        e.storage()
            .get(&ExampleEnum::C(slot))
            .expect("not initialized")
            .unwrap()
    }

    pub fn get_d_symbol(e: Env) -> Symbol {
        e.storage()
            .get(&ExampleEnum::D)
            .expect("not initialized")
            .unwrap()
    }

    pub fn set_deadline(e: Env, val: u64) {
        let env = Env::default();
        let deadline: u64 = env.ledger().timestamp() + val;
        e.storage().set(&ExampleEnum::C(100), &deadline);
    }

    pub fn set_a_address(e: Env, a: Address) {
        e.storage().set(&ExampleEnum::A, &a);
    }

    pub fn set_a_auth(e: Env, a: Address) {
        a.require_auth();
        e.storage().set(&ExampleEnum::A, &a);
    }

    pub fn set_b_address(e: Env, b: Address) {
        e.storage().set(&ExampleEnum::B, &b);
    }

    pub fn set_c_u32(e: Env, slot: u32, c: u32) {
        e.storage().set(&ExampleEnum::C(slot), &c);
    }

    pub fn set_d_symbol(e: Env, d: Symbol) {
        e.storage().set(&ExampleEnum::D, &d);
    }

    pub fn get_current_address() -> Address {
        let env = Env::default();
        let current_address = env.current_contract_address();
        current_address
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
