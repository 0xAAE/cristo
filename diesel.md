# Install diesel toolset
To install the Diesel CLI
```bash
$ cargo install diesel_cli
```
Aftre that `diesel` command will be available to execute

# Project initializing

1. Create a file `src/db.rs` and add the following to the file:
```rust
pub mod models;
pub mod schema;
```

2. Create a sub-directory db. In this directory create an empty file `models.rs`:
```bash
$ mkdir src/db
$ echo > src/db/models.rs
```

3. Create a file `diesel.toml` with the following content:
```toml
[print_schema]
file = "src/db/schema.rs"
```


4. Create the environment file with the `DATABASE_URL` property:
```bash
$ echo DATABASE_URL=dbpath/filename.db > .env
```

5. Add dependencies into the Cargo.toml file:
```toml
[dependencies]
dotenv = "0.15"
tokio = { version = "1.14", features = ["full"] }
diesel = { version = "1.4", features = ["sqlite"] }
uuid = { version = "0.8", features = ["serde", "v4"] }
serde = "1.0"
serde_json = "1.0"
serde_derive = "1.0"
```

1. Initialise Diesel
```bash
$ diesel setup
```

# Data developing

### Create migration scripts
```bash
$ diesel migration generate create_table_name
```
Now you should have a new directory called migrations with two SQL scripts: `up.sql` and `down.sql`.

### Add the contents to up.sql
```sql
CREATE TABLE table_name (
  id INTEGER PRIMARY KEY ASC,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)
```
See: [Data types in SQLite](https://www.sqlite.org/datatype3.html)

### Add the contents to down.sql
```sql
DROP TABLE table_name
```

### Apply the migration
Now that we have defined our table, we can go and create it:
```bash
$ diesel migration run
```
The `src/db/schema.rs` will be updated with new `table!` definition

### Update src/db/models.rs

Add corresponding staff to `src/db/models.rs`
```rust
#[derive(Serialize, Queryable)]
pub struct Asset {
    pub id: u64,
    pub shortcut: String,
    pub fullname: String,
}
```
and
```rust
#[derive(Insertable)]
#[table_name = "assets"]
pub struct NewAsset<'a> {
    pub id: u64,
    pub shortcut: &'a str,
    pub fullname: &'a str,
}
```
