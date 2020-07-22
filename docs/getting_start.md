Getting Start
=====================

To work with the project, you need to install on your system:

- `Rust lang`
- `PostgresSQL`
- `diesel-cli`
- `Redis`
- `Node.js`

The Back-end is developed in the Rust programming language.

Node.js is used to build and optimize code for the Front-end.

PostgresSQL and diesel are needed to work with the database and interact with the Back-end with it.

Redis is used to cache requests, which greatly speeds up server response.


Install
----------------------

Rust: [link](https://www.rust-lang.org/)

PostgresSQL: [link](https://www.postgresql.org/download/)

diesel-cli: [link](http://diesel.rs/) and you get problems for install: [link to stackoverflow](https://stackoverflow.com/questions/1244778/where-do-i-get-libpq-source/61452780#61452780)

Redis: [link](https://redis.io/) and install for windows: [link](https://riptutorial.com/ru/redis/example/29962/%D1%83%D1%81%D1%82%D0%B0%D0%BD%D0%BE%D0%B2%D0%BA%D0%B0-%D0%B8-%D0%B7%D0%B0%D0%BF%D1%83%D1%81%D0%BA-redis-server-%D0%B2-windows)

Node.js: [link](https://nodejs.org/en/) **install LTC version**


Configuration
-----------------------

Before start need change config.

create file `configurations/config.toml` and change database and redis configs

```sh
$ move configurations/config.example.toml > configurations/config.toml
```


Start project
-----------------------

first start:

```sh
$ diesel migration run
$ cargo run
```

run migration for database:

```sh
$ diesel migration run
```

overwriting the database with a new one:

```sh
$ diesel migration redo
```

build and run Back-end:

```sh
$ cargo run
```

build Back-end:
```sh
$ cargo build
```