# auth_service

# Local Started

1. Add database to environment variables:
`export DATABASE_URL=postgresql://postgres:password@localhost:5432/auth`

4. Create database: `cargo sqlx database create`

5. Create migration: `cargo sqlx migrate run`

6. Run the application: `cargo run`

## License

This project is licensed under the [MIT license](LICENSE).
