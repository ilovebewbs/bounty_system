# Bounty system using Rust
**background story**

I always loved John Wick films and admired the system they had in the film that allowed the High Table to put contracts against those who violate the code of conduct, so i thought to myself i should implement such system to both learn more Rust and how to use a database with it - in this case it was mysql - 

i Felt pretty excited after finishing it so i thought i'd share it on here 

**Current features**

- Create, Update, View and delete contracts
- Same CRUD operation but on members (the members of the organization who are allowed to take out the targets as announced)

**Steps to set this up locally**

These instructions are ubuntu specific but WIndows should be similar i hope

- Install mysql using `sudo apt install mysql-server`
- install the diesel cli using `cargo install diesel`
- Clone the repo 
- Create an environment variable called DATABASE_URL and assign your connection string to it and store that in an `.env` file
- Run the migrations `diesel migrations run`
- Launch it with `cargo run`

***CONTRIBUTIONS ARE MORE THAN WELCOME AS WELLL AS ISSUES / FEATURE SUGGESTIONS***

***UNDER MIT LICENSE***