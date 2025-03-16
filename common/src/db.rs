use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::{RunQueryDsl,sql_query};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(url:&str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
}

pub fn run_migrations(conn: &mut PgConnection) {
    let create_user_table_sql = r#"
        CREATE TABLE IF NOT EXISTS account  (
            user_id SERIAL PRIMARY KEY,
            name VARCHAR NULL,
            email VARCHAR UNIQUE NOT NULL,
            password_hash VARCHAR NOT NULL
        );
    "#;

    let create_url_mapping_table_sql = r#"
        CREATE TABLE IF NOT EXISTS url_mapping (
            short_url VARCHAR(10) PRIMARY KEY,
            alias VARCHAR(255) UNIQUE NULL,
            long_url VARCHAR NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            expiration_date TIMESTAMP NULL,
            user_id INT NULL,
            click_count INT DEFAULT 0,
            FOREIGN KEY (user_id) REFERENCES account(user_id) ON DELETE SET NULL
        );
    "#;

    sql_query(create_user_table_sql)
        .execute(conn)
        .expect("Failed to create account table");

    sql_query(create_url_mapping_table_sql)
        .execute(conn)
        .expect("Failed to create url_mapping table");
}