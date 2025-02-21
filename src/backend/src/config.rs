use argon2::Params;

pub const PORT: u16 = 6004;
pub const LISTENING_ON: &str = "0.0.0.0";

pub const DB_PORT: u16 = 5432;
pub const DB_ADDRESS: &str = "hackaton_database";
pub const DB_NAME: &str = "my_database";
pub const DB_USERNAME: &str = "postgres";
pub const DB_PASSWORD: &str = "root"; 



pub const ARGON2_PARAMS: Result<Params, argon2::Error> = Params::new(
    8192, // Memory cost
    1,    // Iterations
    2,    // Parallelism
    None, // Idk what is this tbh
);

