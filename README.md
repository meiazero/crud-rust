# crud-rust
> Simple CRUD application using Rust

## Requirements
- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)

## Setup
1. create a `.env` file in the root directory and add the following:
```env
DATABASE_URL=postgres://username:password@localhost/crud
```

2. Run the following commands:
```bash
cargo build
```

## Run
```bash
cargo run
```
> look the server running at `http://localhost:3333` (by default)

## Endpoints
- `GET /tasks`: list all tasks
- `POST /tasks`: create a new tasks
- `PATCH /tasks/:id`: update a task by id
- `DELETE /tasks/:id`: delete a task by id

## Author
- [meiazero](https://github.com/meiazero?tab=repositories)
