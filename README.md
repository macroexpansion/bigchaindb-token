# BigchainDB Token

## libpq
### MacOS
fish
```bash
brew install libpq
```

## Database
Install `diesel-cli`, requires `libpq`
```bash
cargo install diesel_cli --no-default-features --features postgres
```

### Database Migration
Setup
```bash
diesel setup
```

Create migration
```bash
diesel migration generate create_users
```

Check migration
```bash
diesel migration list
```

Apply migration
```bash
diesel migration run
```

Revert migration
```bash
diesel migration revert
```

(To check `up.sql` and `down.sql` works properly) Redo
```bash
diesel migration redo
```
