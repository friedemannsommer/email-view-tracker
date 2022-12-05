# Email view tracker

> Simple web UI to create tracking URLs for HTML Emails.

## Requirements
* Rust `>=1.65`
* Local (or remote accessible) MySQL / PostgreSQL instance

## Development requirements
* [Docker](https://docs.docker.com/engine/install/)
* [Docker compose](https://docs.docker.com/compose/install/)

## Start locally
1. docker-compose up [mysql, postgres]
2. `cargo run -- start -d (mysql://root:local@127.0.0.1:3310 | postgres://postgres:local@127.0.0.1:3311) -l 127.0.0.1:8080 -v debug -c $COOKIE_SECRET -p $PASSWORD_SECRET`
