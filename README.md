# FerrDB

We welcome any contributions, bug reports, or feature requests!  
Feel free to open an issue or submit a pull request on [GitHub](https://github.com/yourname/ferrdb).  
If you have suggestions or questions, please share them—your feedback helps us improve FerrDB.

A minimal file-based CLI database written in Rust.  
Stores data in `db.json` for persistence across sessions.

## Overview

- **Primary Key Search**: FerrDB uses a B+Tree data structure to perform efficient primary key searches (e.g., for the first column such as "id") with O(log n) performance.
- **Arbitrary Column Search**: For columns other than the primary key (e.g., "name", "age"), FerrDB supports WHERE clause queries by scanning all rows and filtering them. While this full table scan approach works well for small to moderate datasets, users can add secondary indexes later for improved performance.
- **Unified Quote Trimming**: The SQL parser employs a common utility function to remove both single and double quotes from input values, ensuring consistent processing regardless of the quoting style.
- **File-based & CLI**: Data is serialized to `db.json` and a simple CLI allows you to issue SQL-like commands.

## Features
- **CREATE TABLE:** Create new tables with specified columns.
_Example:_ CREATE TABLE users (id, name, age);

- **INSERT INTO:** Insert data into tables. The first column is used as the primary key.  
_Example:_  INSERT INTO users VALUES ('1', 'John', '30'); INSERT INTO users VALUES ('2', 'Mike', '40');

- **SELECT:**  
- **SELECT * FROM <table>:** Retrieve all rows from a table.  
- **SELECT * FROM <table> WHERE <column> = <value>:** Filter rows by matching a column value.  
  _Note:_ Primary key searches use the efficient B+Tree, while non-primary key searches use full table scan filtering.  
  The parser removes surrounding quotes so that both single and double quotes are handled uniformly.
  
## Usage


## Requirements


- **Rust** (Edition 2021 or later)
- **Cargo** for building
- `serde` + `serde_json` (already declared in `Cargo.toml`)


## Build & Run

```bash
# 1. Clone the repository
git clone https://github.com/yourname/ferrdb.git
cd ferrdb

# 2. Build in release mode
cargo build --release

# 3. Run the compiled binary
./target/release/ferrdb
```

> For development, you can also use `cargo run`.

## Quick Start

Once you run the binary (or `cargo run`), you can interact with FerrDB via a simple command-line interface:

1. create a table

```
CREATE TABLE users (id, name, age);
```

2. insert data

```
INSERT INTO users VALUES (1, "John", 30);
```

3. view data

```
SELECT * FROM users;
```

4. exit

```
exit
```


## Usage

The FerrDB CLI supports basic SQL-like commands. Here is a more detailed explanation:

- **Creating a Table:**  
Use the `CREATE TABLE` command followed by the table name and a comma-separated list of column names in parentheses.  
```
CREATE TABLE users (id, name, age);
```

This creates a new table named `users` with the columns `id`, `name`, and `age`.

- **Inserting Data:**  
Use the `INSERT INTO` command followed by the table name and `VALUES`, with the values enclosed in parentheses.  
The first value is used as the primary key and is stored in a B+Tree for fast lookup.  
```
INSERT INTO users VALUES ('1', 'John', '30'); 
INSERT INTO users VALUES ('2', 'Mike', '40');
``` 

- **Selecting Data:**  
- **Full Table Query:**  
  Use the `SELECT * FROM <table>;` command to retrieve all rows from the specified table.  
  _Example:_  
  ```
  SELECT * FROM users;
  ```
- **Filtered Query (WHERE Clause):**  
  Use the `SELECT * FROM <table> WHERE <column> = <value>;` command to filter rows based on a column value.  
  The parser removes surrounding quotes from the value, so both single and double quotes work.  
  _Examples:_  
  ```
  SELECT * FROM users WHERE name = 'John';
  SELECT * FROM users WHERE name = "Mike";
  ```
  For primary key searches (usually the first column), the B+Tree search is used, providing efficient lookup. For other columns, all rows are scanned and filtered.

## Example Session
```
Welcome to SimpleRDB CLI. Type 'exit' to quit.
> CREATE TABLE users (id, name, age);
Table `users` created.
> INSERT INTO users VALUES (1, "John", 30);
Data inserted into 'users'.
> SELECT * FROM users;
Table 'users':
[["id", "name", "age"]
["1", "John", "30"]]
> exit
Goodby!
```

When you restart the application, `db.json` will be loaded, and any previously inserted rows are available.

## Docker (Optional)

If you wish to run FerrDB within a Docker environment:

1. Create a `Dockerfile` that installs Rust and copies this repository.
2. (Optionally) use Docker Compose to mount your working directory for an interactive dev environment.
3. Build and run the container, then manually execute:
```bash
cargo build --release
./target/release/ferrdb
```

MIT License

Copyright (c) 2025 hnk

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
