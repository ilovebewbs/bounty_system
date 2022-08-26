#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod args;
pub mod ops;
pub mod db;
pub mod schema;
pub mod models;
use args::TransactionType;
use args::BountyArgs;
use clap::Parser;
use ops::contract_op::handle_contract;
use ops::member_ops::handle_member;
fn main() {
    let args = BountyArgs::parse();
    match args.transaction_type {
        TransactionType::Contract(contract) => handle_contract(contract),
        TransactionType::Member(member) => handle_member(member),
    }
}
