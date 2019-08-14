#[derive(Clone)]
pub struct State {
    pub pool : r2d2::Pool<r2d2_postgres::PostgresConnectionManager>,
    pub schema : String,
}

impl State {
    pub fn new(opt: &crate::Opt)
    -> State
    {
        let m = r2d2_postgres::PostgresConnectionManager::new (
            &*(opt.postgres),
            r2d2_postgres::TlsMode::None,
        ).expect("Failed to connect to db");

        let p = r2d2::Pool::new(m).unwrap();

        State {
            pool: p,
            schema: opt.schema.to_string(),
        }
    }

    pub fn get_connection(&self)
    -> r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>
    {
        self.pool.get().unwrap()
    }

    pub fn get_sql(&self, name:&str) -> String {
        format!("select {schema}.{name}($1::jsonb)",
            schema=self.schema, name=name)
    }
}
