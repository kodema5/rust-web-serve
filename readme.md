# rust web serve

```
web-serve 0.1.0
kodema5 <kodema5@outlook.com>

GETs returns static-files from [dir]
POSTs calls [postgres] [schema].[pg_function_name](input)
stores .key entries in [redis]

EXAMPLE:
    docker run --rm -d -p 5432:5432 -v %cd%:/work --name pg1 -e POSTGRES_PASSWORD=rei postgres
    docker exec -it pg1 psql -U postgres -d postgres -f /work/index.sql

    docker run --rm -d -p 6379:6379 --name rdb1 redis

    web-serve
    (access http://localhost:8000/index.html)

USAGE:
    web-serve.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dir <dir>               [default: .]
        --get <get_path>          [default: /]
    -p, --port <port>             [default: 8000]
        --post <post_path>        [default: /{pg_function_name}]
        --postgres <postgres>     [default: postgres://postgres:rei@localhost:5432/postgres]
        --redis <redis>           [default: redis://localhost]
    -s, --schema <schema>         [default: web]
```


## on .key entries in redis

.key is a key prefixed with '.'.
values of `.key` in payload is retrieved from redis;
while values of `.key` in output is stored in redis.

```
payload = { .key: field_name, ... }
input   = { field_name: redis(.key), ... }

output = select schema.pg_function_name(input)
       = { .key: value, ... }

redis(.key) = value;
response = { ... }
```

## future

- postgres and redis pool parameters
- options of redis operations to set/get .key (hset, etc)