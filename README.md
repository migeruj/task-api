rocket-sqlite

# Database configuration

# Install Diesel

1. For Ubuntu/Debian Install libmysqlclient-dev package, for windows install mysql-client binaries. Execute ```apt install libmysqlclient-dev ```
2. Config ```DATABASE_URL``` environment variable with your mysql URI database location
3. Execute ```cargo install diesel_cli --no-default-features --features mysql```


# Run

1. Execute ```diesel migration run``` for execute migrations to create the project database
2. Execute ```cargo run ``` at cargo.toml level.
3. Now you can use the api at http://localhost:8000

# Tests