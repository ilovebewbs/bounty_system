extern crate diesel;
use crate::args::{CreateContract, DeleteContract, UpdateContract, ContractCommand, ContractOps};
use crate::db::establish_connection;
use crate::models::*;
use crate::schema::contracts;
use diesel::prelude::*;

pub fn handle_contract(contract: ContractCommand) {
    match contract.command {
        ContractOps::Create(contract) => create_contract(contract),
        ContractOps::Update(contract) => update_contract(contract),
        ContractOps::View => view_contracts(),
        ContractOps::Delete(contract) => delete_contract(contract), 
    }
}

pub fn create_contract(contract: CreateContract) {
    let newcontract = NewContract {
        the_owner: &contract.owner,
        the_target: &contract.target,
        bounty: contract.bounty,
    };
    let conn = establish_connection();
    diesel::insert_into(contracts::table)
        .values(newcontract)
        .execute(&conn)
        .expect("failed to add a new contract");
    println!("✅ contract has been successfully added");
}

pub fn update_contract(contract: UpdateContract) {
    use crate::schema::contracts::dsl::*;
    let conn = establish_connection();
    let other_id = contract.id;
    let new_contract = NewContract {
        the_owner: &contract.owner,
        the_target: &contract.target,
        bounty: contract.bounty,
    };
    diesel::update(contracts.filter(id.eq(other_id))).set(new_contract).execute(&conn).expect("failed to update the contract");
    println!("✅ contract with id; {} has been updated", other_id);

}

pub fn delete_contract(contract: DeleteContract) {
    use crate::schema::contracts::dsl::*;
    let conn = establish_connection();
    let other_id = contract.id;
    diesel::delete(contracts.filter(id.eq(other_id)))
        .execute(&conn)
        .expect("failed to delete contract");
    println!("🗑️ contract with id: {} has been deleted", other_id);
}

pub fn view_contracts() {
    use crate::schema::contracts::dsl::*;
    let connection = establish_connection();
    let results = contracts
        .load::<Contract>(&connection)
        .expect("faied to load contracts");
    for x in results {
        println!("{:#?}", x);
    }
}
