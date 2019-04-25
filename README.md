# Graphy

A Rust GraphQL server starter project.

## Setup

Add secrets to `secrets/` directory:

- `db_user.txt`
- `db_password.txt`
- `db_name.txt`

By Docker convention sensitive information is passed by
secrets and is read by file, whilst non-sensitive
is passed by a normal arg.

### Config

Environmental specific arguments (such as `db_password`)
are passed in at runtime via CLI `args` and
parsed in `Config`.

Project specific arguments are

### For development

Start PostgreSQL (Dockerized):

```bash
docker-compose up db
```

Start API server (locally):

```bash
cargo run -- --db-name-file="../secrets/db_name.txt" --db-password-file="../secrets/db_password.txt" --db-user-file="../secrets/db_user.txt"
```

Initialise database (Diesel CLI):

```bash
docker run --rm -v "$(pwd)":/app --network="<project_name>_default" willsquire/diesel-cli --database-url="postgres://<db_user>:<db_password>@db/<db_name>" setup
```
