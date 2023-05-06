# blog_api

This is a simple blog api written in rust. I used it to try out some REST-API development in rust. It uses [Rocket](https://rocket.rs/) and [Diesel](https://diesel.rs/). Diesel uses a postgres database. It is configured to use a database called `blog_api` that has a user called `blog_api` with the password `password`. I know super secure :smile:.

To use it you first have to run the diesel migrations after installing the [diesel_cli](https://diesel.rs/guides/getting-started).

``` bash
diesel migration run
```

Then simply run

``` bash
cargo run
```
