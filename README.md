# Surreal DB and Actix Web

### Features

- User creation
- User authentication
- Middleware for authorization

### Database

This app utilizes SurrealDB as its database. It's an opportunity for me to explore this innovative database.

### Routing

- `/`
- `/api`
  - `/auth`
    - `/login`
    - `/register`
  - `/v1` ➜ **TODO!**
  - `/admin` _admin_only!_
    - `/users`

### Getting Started

To run this application, follow these steps:

1. Install SurrealDB server and start it.
2. Copy the contents from `env.example` and create a new file named `.env`. Fill in the required environment variables.
3. Execute `cargo run` or, if you have Cargo Watch installed, use `cargo watch -x "run"` to begin.

Enjoy exploring and learning with this project!

### Getting Started

To run this application, follow these steps:

1. Install SurrealDB server and start it.
2. Copy the contents from `env.example` and create a new file named `.env`. Fill in the required environment variables.
3. Execute `cargo run` or, if you have Cargo Watch installed, use `cargo watch -x "run"` to begin.

Enjoy exploring and learning with this project!

## Important

If you are testing the api with something like Postman, RapidApiClient, Dont forget to set the cookie secure to false and SameSite to lax in the `session.rs` so that it sets the cookie correcty file or I will add a env variable for it.
If you are testing it in a client side app on another origin then you should leave the `SameSite: None` and the session cookie secure to false. Otherwise the browser will not allow to set the cookie.
