# Graphy

A Rust GraphQL server starter project.

## Quick start

Clone repo with `project-name`:

```bash
git clone git@github.com:WillSquire/Graphy.git \
  project-name \
  --depth=1 \
  --branch=master
```

Create `/secrets` directory in project root with these files and their values:

- `db_user.txt`
- `db_password.txt`
- `db_name.txt`
- `hash_salt.txt`
- `token_secret.txt`

Run the stack on Docker:

```bash
docker-compose up
```

## Manual

### Config

Any sensitive data is passed at runtime either by value or file `args`.

By value:

```bash
cargo run -- \
  --db-user="..." \
  --db-password="..." \
  --db-name="..." \
  --db-server="..." \ # Optional
  --hash-salt="..." \
  --token-secret="..."
```

By file:

```bash
cargo run -- \
  --db-user-file="../secrets/db_user.txt" \
  --db-password-file="../secrets/db_password.txt" \
  --db-name-file="../secrets/db_name.txt" \
  --hash-salt-file="../secrets/hash_salt.txt" \
  --token-secret-file="../secrets/token_secret.txt"
```

Note these are passed at runtime to avoid retrieval through decompilers. It's recommend that these are passed by file if using Docker via secrets. The `Dockerfile` does this already.

Project specific arguments are located with that feature
(i.e. not in `Config`).

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
