# AMQP-tester

## Build

You need rust and cargo installed.

```bash
cargo build --release
```


## Usage

```
Usage:
    ./target/release/amqp-tester [OPTIONS]

Fetch messages form rabbitmq.

optional arguments:
  -h,--help             show this help message and exit
  -V,--version          Show version
  -U,--url URL          Url to rabbitmq
  -Q,--queue QUEUE      Queue name
  -C,--count COUNT      Prefetch count
  ```
