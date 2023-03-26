# Email view tracker

> Simple web UI to create tracking URLs for HTML Emails.

<details>
<summary>Screenshots</summary>

### Login

![Screenshot of the login page](screenshot/login.png)

### Home

![Screenshot of the home page on page 1](screenshot/home-page-1.png)
![Screenshot of the home page on page 2](screenshot/home-page-2.png)

### Tracker

#### Create

![Screenshot of the tracker creation page](screenshot/tracker-create.png)

#### Edit

![Screenshot of the tracker update page](screenshot/tracker-edit.png)

### Profile

![Screenshot of the profile update page](screenshot/profile-edit.png)
</details>

## Requirements

* Rust `>=1.65`
* Local (or remote accessible) MySQL / PostgreSQL instance

## Development requirements

* [Docker](https://docs.docker.com/engine/install/)
* [Docker compose](https://docs.docker.com/compose/install/)

## Start locally

1. docker-compose up [mysql, postgres]
2. `cargo run -- migrate run -d (mysql://root:local@127.0.0.1:3310 | postgres://postgres:local@127.0.0.1:3311) -v debug`
3. `cargo run -- user create -d postgres://postgres:local@127.0.0.1:3311 -v debug -p $PASSWORD_SECRET -P $USER_PASSWORD -u $USERNAME`
4. `cargo run -- start -d (mysql://root:local@127.0.0.1:3310 | postgres://postgres:local@127.0.0.1:3311) -l 127.0.0.1:8080 -v debug -c $COOKIE_SECRET -p $PASSWORD_SECRET`

## Deploy

1. Download pre-built binary or build from source
2. `email-view-tracker migrate run -d $DATABASE_URL`
3. `email-view-tracker user create -d $DATABASE_URL -p $PASSWORD_SECRET -P $USER_PASSWORD -u $USERNAME`
4. `email-view-tracker start -d $DATABASE_URL -l $BIND_ADDRESS -c $COOKIE_SECRET -p $PASSWORD_SECRET`

## Open source licenses

A list of licenses for the projects used in "Email view tracker" can be found
here: [friedemannsommer.github.io/email-view-tracker/licenses.html](https://friedemannsommer.github.io/email-view-tracker/licenses.html).

This product includes software developed by the OpenSSL Project for use in the OpenSSL
Toolkit. ([www.openssl.org](https://www.openssl.org/))
