# Warp API | Rust

## Overview

This project demonstrates a web API built using Warp and Rust. The API uses PostgreSQL for database operations and includes basic user management functionalities with password encryption and JWT authentication. Environment variables are used for configuration, and Warp provides an efficient, asynchronous web framework for handling requests.

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── controllers
│   │   └── mod.rs
│   ├── errors
│   │   └── mod.rs
│   ├── helpers
│   │   └── mod.rs
│   ├── main.rs
│   ├── middlewares
│   │   └── mod.rs
│   └── routes
│       └── mod.rs
└── todo.md
```

## Features

- **Password Encryption:** Uses Argon2 for securely hashing and verifying passwords.
- **JWT Authentication:** Implements JSON Web Tokens (JWT) for secure user authentication.

## Database

This project uses PostgreSQL for database operations. To run PostgreSQL in a Docker container, use the following command:

```bash
docker run -e POSTGRES_PASSWORD=<password> -p 5432:5432 -d postgres
```

This command sets up a PostgreSQL instance with the password choosed, mapping port 5432 on the container to port 5432 on your host machine.

## Configuration

To configure the project, follow these steps:

1. **Create a `.env` file in the project root** and add the necessary environment variables:

```env
ADMIN_PASSWORD='12345'
DATABASE_PASSWORD='12345'
JWT_SECRET='12345'
```

2. The `DATABASE_PASSWORD` variable specifies the password for PostgreSQL connection. The `JWT_SECRET` is used for signing JWTs. The `ADMIN_PASSWORD` is used to create an initial admin user.

## Running Locally

To run the project locally, follow these steps:

1. **Install project dependencies** using Cargo:

```bash
cargo build
```

2. **Run the server**:

```bash
cargo run
```

## Endpoints

This project provides the following API endpoints:

| Endpoint                  | Description                                             | HTTP Method |
| ------------------------- | ------------------------------------------------------- | ----------- |
| `/authenticate`           | User authenticate endpoint, requires email and password | POST        |
| `/status`                 | Check server status                                     | GET         |
| `/users/create_user`      | Create a new user                                       | POST        |
| `/users/delete_user/{id}` | Delete a user by id                                     | DELETE      |
| `/users/get_users`        | Retrieve a list of all users (admin only)               | GET         |
| `/users/update_user/{id}` | Update a user by id                                     | PUT         |

### JWT Authentication

- **`/authenticate`**: Provides a JWT token upon successful authentication. The token must be included in the `Authorization` header for requests to protected endpoints.

- **Protected Endpoints**: The `/users/get_users` endpoint requires the user to be authenticated.

## Images

![image](https://github.com/user-attachments/assets/55652a7c-e4d9-4f90-b6d7-3c35da99fe33)

![image](https://github.com/user-attachments/assets/e4b7cbfc-0b11-4951-8520-fc243f4b4201)

![image](https://github.com/user-attachments/assets/eaa49406-c2c1-4b33-9659-b37e90cebf8c)

![image](https://github.com/user-attachments/assets/36f25ca5-86ad-4452-9f9e-2d0cbc1d2b4b)



