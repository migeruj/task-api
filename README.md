# Install Rust - Nightly Version
1. Install [rustup](https://rustup.rs/#). You can use the following command for UNIX / Ubuntu / WSL
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    
2.  Rust nightly as your default toolchain by running the command:
    ```bash
    rustup default nightly
    ```
3. Enjoy 🚀

# Database configuration
## Install Diesel
##### Diesel is the best ORM currently in Rust. Check the [page](http://diesel.rs/) for code examples and overviews 
1. For Ubuntu/Debian Install libmysqlclient-dev package, for windows install mysql-client binaries.
    ```bash 
    apt install libmysqlclient-dev
    ```
2. Config ```DATABASE_URL``` environment variable with your mysql URI database location [URI Example](https://docs.rs/diesel/1.4.5/diesel/mysql/struct.MysqlConnection.html)
3. Execute 
    ```bash 
    cargo install diesel_cli --no-default-features --features mysql
    ```
    
# Run

1. Execute ```diesel migration run``` for execute migrations to create the project database
2. Execute ```cargo run ``` at cargo.toml level.
3. Now you can use the api at http://localhost:8000

# Tests