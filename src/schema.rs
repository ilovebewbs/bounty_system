table! {
    contracts (id) {
        id -> Integer,
        the_owner -> Varchar,
        the_target -> Varchar,
        bounty -> Integer,
        finished -> Bool,
    }
}

table! {
    members (id) {
        id -> Integer,
        fullname -> Varchar,
        city -> Varchar,
        phone -> Bigint,
    }
}

allow_tables_to_appear_in_same_query!(
    contracts,
    members,
);
