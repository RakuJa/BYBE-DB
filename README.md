![Rust](https://img.shields.io/badge/Rust-664666?style=for-the-badge&logo=rust&logoColor=red)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

# BYBE - DB

> Beyond Your Bestiary Explorer (BYBE) provides tools to help Pathfinder 2e Game Masters. Built as the database initializer of [BYBE - Backend](https://github.com/RakuJa/BYBE/)

This Rust program downloads Foundry VTT source data, parses it, and stores it in a PostgreSQL database. A pre-built dump (`db/bybe_postgres.sql`) is included to restore data and avoid losing legacy creatures or change ids.

---

## Docker (recommended)

### First start

Builds the images, restores the dump into PostgreSQL, and runs the updater once:

```bash
docker compose up --build
```

### Start the DB only (serve queries)
The previous step should have already started the db, if it did not run:
```bash
docker compose up db
```

### Re-run the updater (if needed)

Fetches the latest Foundry data and repopulates the database:
```bash
docker compose run --rm updater
```

After this, [refresh the dump](#refreshing-the-dump) so the new data is captured.

---

## Local setup

**Requirements:** [Rust](https://www.rust-lang.org/tools/install), a running PostgreSQL instance.

1. Copy and fill in the environment file:

```bash
cp .env.example .env
```

2. Run schema migrations:

```bash
sqlx migrate run
```

3. Run the data updater:

```bash
cargo run --release
```

The binary applies any pending migrations automatically, then clears and repopulates all data tables. The source repo is cloned on first run (~slow); subsequent runs reuse the existing clone (~1 minute).

---

## Refreshing the dump

Run this after the updater finishes to capture the new state into `db/bybe_postgres.sql`:

```bash
docker exec bybe-postgres pg_dump -U postgres bybe > db/bybe_postgres.sql
```

Commit `db/bybe_postgres.sql` so the next `docker compose up --build` starts from the updated data without re-running the pipeline.
## Exporte pglite compatible sql

```bash
docker exec bybe-postgres pg_dump -U postgres --format=plain --no-owner --no-privileges --inserts bybe > db/bybe_pglite.sql
```
## Support me

If you like this tool, consider supporting me:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/rakuja)
