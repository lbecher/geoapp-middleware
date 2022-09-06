use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Conn {
    server: String,
    database: String,
    username: String,
    password: String,
}

pub fn getpath(conn: Conn) -> String {
    let path: String = format!("postgresql://{}:{}@{}/{}",
        conn.username.as_str(),
        conn.password.as_str(),
        conn.server.as_str(),
        conn.database.as_str()
    );
    return path;
}