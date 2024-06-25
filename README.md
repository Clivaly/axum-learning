### Learning Axum

`
useful links:
`
   - [Install Rust](https://www.rust-lang.org/tools/install)
   - [Rust Book](https://doc.rust-lang.org/book/title-page.html)
   - [Rust by Example](https://doc.rust-lang.org/rust-by-example)
   - [Visual Studio Code](https://code.visualstudio.com/download)
   - [Git](https://git-scm.com/downloads)

`
Tip: I'm using an old version of axum, stay tuned! I will update later.
`

#### Create a new rust project, type in the terminal:

    $ cargo new axum-learn
    $ cd axum-learn
    $ code .

#### Add Axum and tokio and some tokio features to the project and do run:
    $ cargo add axum tokio
    $ cargo add tokio -F macros -F rt-multi-thread
    $ cargo run


#### Add auto restarting to the project and run:
    $ cargo install cargo-watch
    $ cargo watch -x run

#### Run local documentation:
    $ cargo doc
    $ cargo doc --open

