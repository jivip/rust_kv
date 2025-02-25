# rust_kv

A simple key-value store using a txt file

It will create a new txt file called db.txt on first use.
This text file db.txt is used to store and retrieve the key value pairs.

`cargo run -- set KEY:VALUE` stores a `KEY:VALUE` pair in `db.txt`.  
`cargo run -- get KEY` retrieves a `KEY:VALUE` pair from `db.txt`.  
`cargo run -- del KEY` deletes a `KEY:VALUE` pair from `db.txt`.
