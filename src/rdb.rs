#[derive(Clone)]
pub struct State {
    pub pool : r2d2::Pool<r2d2_redis::RedisConnectionManager>,
}

impl State {
    pub fn new(opt: &crate::Opt)
    -> State
    {
        let m = r2d2_redis::RedisConnectionManager::new(
            &*(opt.redis),
        ).expect("Failed to connect to rdb");

        let p = r2d2::Pool::new(m).unwrap();

        State {
            pool: p
        }
    }

    pub fn get_connection(&self)
    -> r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>
    {
        self.pool.get().unwrap()
    }
}
