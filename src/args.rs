use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
#[clap(author="@ilovebewbs on github", version)]
/// A simple bounty system inspired by John Wick
/// You can create, update, view or delete contracts
/// You can also add members to the table that are allowed to take those contracts against targets
pub struct BountyArgs{
    #[clap(subcommand)]
    /// add, update, delete a contract
    pub transaction_type: TransactionType
}

#[derive(Debug,Subcommand)]
pub enum TransactionType{
    Contract(ContractCommand),
    Member(MemberCommand)
}

#[derive(Debug, Args)]
pub struct ContractCommand{
    #[clap(subcommand)]
    pub command: ContractOps
}

#[derive(Debug, Subcommand)]
pub enum ContractOps{
    Create(CreateContract),
    Update(UpdateContract),
    View,
    Delete(DeleteContract),
}

#[derive(Debug, Args)]
pub struct MemberCommand{
    #[clap(subcommand)]
    pub command: MemberOps
}

#[derive(Debug, Subcommand)]
pub enum MemberOps{
    Create(CreateMember),
    Update(UpdateMember),
    View,
    Delete(DeleteMember)
}

#[derive(Debug, Args)]
pub struct CreateContract{
    /// who created the contract
    pub owner: String,
     /// the target of the contract
    pub target: String,
    /// the reward of the contract
    pub bounty: i32,
   
}

#[derive(Debug, Args)]
pub struct UpdateContract{
    pub id: i32,
    pub owner: String,
    pub target: String,
    pub bounty: i32,
}

#[derive(Debug, Args)]
pub struct DeleteContract{
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct CreateMember{
    pub fullname: String,
    pub city: String,
    pub phone: i64   
}

#[derive(Debug, Args)]
pub struct UpdateMember{
    pub id: i32,
    pub fullname: String,
    pub city: String,
    pub phone: i64,
}

#[derive(Debug, Args)]
pub struct DeleteMember{
    pub id: i32
}