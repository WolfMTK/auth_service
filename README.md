# auth_service

# Local Started

1. Add database to environment variables: `export DATABASE_URL=postgresql://postgres:password@localhost:5432/auth`

2. Create database: `cargo sqlx database create`

3. Create migration: `cargo sqlx migrate run`

4. Run the application: `cargo run`

## License

This project is licensed under the [MIT license](LICENSE).
