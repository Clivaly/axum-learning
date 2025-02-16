### Learning Axum

`
useful links:
`
   - [Install Rust](https://www.rust-lang.org/tools/install)
   - [Rust Book](https://doc.rust-lang.org/book/title-page.html)
   - [Rust by Example](https://doc.rust-lang.org/rust-by-example)
   - [Visual Studio Code](https://code.visualstudio.com/download)
   - [Git](https://git-scm.com/downloads)
   - [SeaORM](https://www.sea-ql.org/SeaORM)

#### Create a new rust project, type in the terminal:

    $ cargo new axum-learn
    $ cd axum-learn
    $ code .

### Crates:
#### Add Axum and tokio and some tokio features to the project and do run:
    $ cargo add axum
    $ cargo add axum -F headers  # check axum-extra
    $ cargo add tokio -F macros -F rt-multi-thread
    $ cargo add serde -F derive
    $ cargo run

#### Add auto restarting to the project and run:
    $ cargo install cargo-watch
    $ cargo watch -x run

#### Add tower for middlewares (active cors):
    $ cargo add tower-http -F cors

#### Add Valiator crate:
    $ cargo add validator -F derive

#### Add sea-ORM:
    $ cargo add sea-orm -F sqlx-postgres -F runtime-tokio-rustls

#### Add .env:
    $ cargo add dotenvy dotenvy_macro 

#### Add Chrono crate:
    $ cargo add chrono

#### Add Bcrypt crate:
    & cargo add bcrypt

#### Add Jsonwebtoken crate:
    & cargo add jsonwebtoken

#### Run local documentation:
    $ cargo doc
    $ cargo doc --open

#### Add extras crate:
    $ cargo add axum-extra -F typed-header

