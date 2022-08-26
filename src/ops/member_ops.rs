extern crate diesel;
use crate::args::{CreateMember, DeleteMember, UpdateMember, MemberCommand, MemberOps};
use crate::db::establish_connection;
use crate::models::*;
use crate::schema::members;
use diesel::prelude::*;

pub fn handle_member(member: MemberCommand) {
    match member.command {
        MemberOps::Create(member) => create_member(member),
        MemberOps::Update(member) => update_member(member),
        MemberOps::View => view_members(),
        MemberOps::Delete(member) => delete_member(member), 
    }
}

pub fn create_member(member: CreateMember) {
    let newmember = NewMember {
        fullname: &member.fullname,
        city: &member.city,
        phone: member.phone,
    };
    let conn = establish_connection();
    diesel::insert_into(members::table)
        .values(newmember)
        .execute(&conn)
        .expect("failed to add a new member");
    println!("✅ member has been successfully added");
}

pub fn update_member(member: UpdateMember) {
    use crate::schema::members::dsl::*;
    let conn = establish_connection();
    let other_id = member.id;
    let new_member = NewMember {
       fullname: &member.fullname,
       city: &member.city,
       phone: member.phone
    };
    diesel::update(members.filter(id.eq(other_id))).set(new_member).execute(&conn).expect("failed to update the member");
    println!("✅ member with id; {} has been updated", other_id);

}

pub fn delete_member(member: DeleteMember) {
    use crate::schema::members::dsl::*;
    let conn = establish_connection();
    let other_id = member.id;
    diesel::delete(members.filter(id.eq(other_id)))
        .execute(&conn)
        .expect("failed to delete member");
    println!("🗑️ member with id: {} has been deleted", other_id);
}

pub fn view_members() {
    use crate::schema::members::dsl::*;
    let connection = establish_connection();
    let results = members
        .load::<Member>(&connection)
        .expect("faied to load members");
    for x in results {
        println!("{:#?}", x);
    }
}
