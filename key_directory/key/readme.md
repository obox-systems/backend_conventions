# Keys

This document provides a concise example of an environment configuration script, used to set up environment variables for a project. These variables configure application behavior without altering the code.

## Example

Here's an example of a script named `-env.sh`:

```bash
DATABASE_URL=postgres://postgres:password@localhost/database
SERVER_ADDR='127.0.0.1:8080'
```

- `DATABASE_URL`: Connection string for a PostgreSQL database, including username, password, host, and database name.
- `SERVER_ADDR`: Specifies the server's IP address and port.

## How to Run

To apply these variables to your current shell session, use:

```bash
. ./key/-env.sh
```

This command sources the script, making the variables available in your current session. Ensure `-env.sh` is in the `key` directory relative to your current location.
