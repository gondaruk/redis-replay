# redis-replay

Small utility to replay commands to redis from `MONITOR` output.

## CLI
```shell
USAGE:
    redis-replay [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -s, --silent     Be silent (log only errors)
    -V, --version    Prints version information
    -v, --verbose    Be verbose (log debug)

OPTIONS:
    -i, --input <input>    Path to to file with MONITOR output
    -r, --redis <redis>    Redis connection string, e.g. redis://127.0.0.1:6379 [default: redis://127.0.0.1:6379]
```

## Example
```shell
redis-replay -s -i ./path/to/file.yml
```
