# FerrDB

We welcome any contributions, bug reports, or feature requests!  
Feel free to open an issue or submit a pull request on [GitHub](https://github.com/yourname/ferrdb).  
If you have suggestions or questions, please share them—your feedback helps us improve FerrDB.

A minimal file-based CLI database written in Rust.  
Stores data in `b.json` for persistence across sessions.

## Overview

- **File-based**: Data is serialized to `db.json` using `serde_json`.
- **CLI-based**: Operate with simple SQL-like commands (`CREATE TABLE`, `INSERT INTO`, `SELECT`, etc.).
- **Persistent**: Inserted data is saved to disk and reloaded on next startup.



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

```praintf
SELECT * FROM users;
```

4. exit

```praintf
exit
```


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
