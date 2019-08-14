use actix_web::{web, App, HttpServer};
mod post;

fn main() {
    let opt = Opt::from_args();
    println!("
serving at 127.0.0.1:{port}
press ^C to terminate
    ", port=opt.port);

    {
        let s = State::new(&opt);

        let post_path = opt.post_path;
        let get_path = opt.get_path;
        let dir = opt.dir;

        HttpServer::new(move || {
            let static_files = actix_files::Files::new(
                get_path.clone().as_str(),
                dir.clone().as_str());

            App::new()
            .data(s.clone())
            .route(
                post_path.clone().as_str(),
                web::post().to(post::index))
            .service(static_files)
        })
    }
    .bind(format!("127.0.0.1:{}", opt.port))
    .expect(&format!("Can not bind to port {}", opt.port))
    .run()
    .unwrap();
}


// https://docs.rs/structopt/0.2.18/structopt/
use structopt::StructOpt;
#[derive(Clone, Debug, StructOpt)]
#[structopt(name="web-serve", about="
GETs returns static-files from [dir]
POSTs calls [postgres] [schema].[pg_function_name](input)
stores .key entries in [redis]

EXAMPLE:
    docker run --rm -d -p 5432:5432 -v %cd%:/work --name pg1 -e POSTGRES_PASSWORD=rei postgres
    docker exec -it pg1 psql -U postgres -d postgres -f /work/index.sql

    docker run --rm -d -p 6379:6379 --name rdb1 redis

    web-serve
    (access http://localhost:8000/index.html)
")]
pub struct Opt {

    #[structopt(short="p", long="port", default_value="8000")]
    pub port: i32,

    #[structopt(short="d", long="dir", default_value=".")]
    pub dir: String,

    #[structopt(long="get", default_value="/")]
    pub get_path: String,

    #[structopt(long="post", default_value="/{pg_function_name}")]
    pub post_path: String,

    #[structopt(long="postgres", default_value="postgres://postgres:rei@localhost:5432/postgres")]
    pub postgres: String,

    #[structopt(short="s", long="schema", default_value="web")]
    pub schema: String,

    #[structopt(long="redis", default_value="redis://localhost")]
    pub redis: String,
}

mod pg;
mod rdb;
#[derive(Clone)]
pub struct State {
    pub postgres : pg::State,
    pub redis: rdb::State,
}

impl State {
    pub fn new(opt: &Opt) -> State {
        State {
            postgres: pg::State::new(&opt),
            redis: rdb::State::new(&opt),
        }
    }
}

