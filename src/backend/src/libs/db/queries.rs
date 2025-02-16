pub const QUERIES: [&str; 1] = [
    r#"
        CREATE TABLE IF NOT EXISTS Score (

            score_id SERIAL PRIMARY KEY NOT NULL,
            score SMALLINT NOT NULL,
            signature VARCHAR(3) NOT NULL,
            time_played INT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        
        );
    "#
];