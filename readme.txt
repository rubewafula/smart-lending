# Resposnibilities

main.rs: Starts the Axum server, builds the router, and loads environment/configs.

routes/: Contains route definitions (e.g., /auth, /transactions).

handlers/: Business logic for each route.

models/: Data structures for users, transactions, accounts, etc. Shared with DB + APIs.

db/: Connection pooling, query logic, database abstraction.

config.rs: Loads environment variables (e.g., DB credentials, JWT secrets).

error.rs: Custom error types and global error handling.

utils.rs: JWT, password hashing, logging utilities, etc.
