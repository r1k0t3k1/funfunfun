use sqlx::{PgPool, postgres::PgConnectOptions};

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl From<DatabaseConfig> for PgConnectOptions {
    fn from(value: DatabaseConfig) -> Self {
        Self::new()
            .host(&value.host)
            .port(value.port)
            .username(&value.username)
            .password(&value.password)
            .database(&value.database)
    }
}

impl DatabaseConfig {
    // TODO Configファイルから設定値を読み込むようにする
    pub fn new() -> Self {
        Self {
            host: "localhost".into(),
            port: 5432,
            username: "f3".into(),
            password: "funfunfun".into(),
            database: "f3".into(),
        }
    }

    pub fn connect_database_with(self) -> PgPool {
        PgPool::connect_lazy_with(self.into())
    }
}
