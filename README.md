<div align = center>

![][views] ![][stars] ![][forks] ![][issues] ![][license] ![][code-size] ![][commit-activity]

# JioSaavn API

### A wrapper for JioSaavn API written in Rust programming language 🦀.

> [!NOTE]
> This project is for educational purposes only. The author of this project is not responsible for any misuse of this API. Use it at your own risk.

## 🔨 Local Development

</div>

- Rust should be installed on your system. If not, then install it from [here](https://www.rust-lang.org/tools/install).

```sh
git clone https://github.com/rajput-hemant/jiosaavn-api-rs
cd jiosaavn-api-rs
```

- Run the following command to run the project.

```sh
cargo run
```

- Run the following command to build the project.

```sh
cargo build
```

## Deploying your own instance

### [Shuttle.rs](https://shuttle.rs/)

- Install the [Shuttle CLI](https://github.com/shuttle-hq/shuttle/releases)

```
cargo install cargo-shuttle
```

- Make sure to change the `main.rs` file to the following:

```rust
// #[tokio::main]
#[shuttle_runtime::main]
// async fn main() {
async fn axum() -> shuttle_axum::ShuttleAxum {
    // tracing_subscriber::fmt::init();

    let router = Router::new()
    ...

    // let addr = "[::]:8080".parse().unwrap();

    // tracing::debug!("🚀 Server listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    Ok(router.into())
}
```

```
cargo shuttle start
cargo shuttle deploy --allow-dirty
```

### [Fly.io](https://fly.io/)

- Install the [Fly CLI](https://fly.io/docs/hands-on/install-flyctl)

```
curl -L https://fly.io/install.sh | sh
```

- A `fly.toml` file will be automatically generated. `fly launch` comman which will ask a few questions to set everything up.

```
fly launch --name jiosaavn
```

- Deploy the app

```
fly deploy
```

<div align = center>

## 📜 License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## 🦾 Contributors:

<a href="https://github.com/rajput-hemant/jiosaavn-api-rs/graphs/contributors" target="blank"> <img src="https://contrib.rocks/image?repo=rajput-hemant/jiosaavn-api-rs&max=500" />

</div>

<!----------------------------------{ Labels }--------------------------------->

[views]: https://komarev.com/ghpvc/?username=jiosaavn-api-rs&label=view%20counter&color=red&style=flat
[code-size]: https://img.shields.io/github/languages/code-size/rajput-hemant/jiosaavn-api-rs
[issues]: https://img.shields.io/github/issues-raw/rajput-hemant/jiosaavn-api-rs
[license]: https://img.shields.io/github/license/rajput-hemant/jiosaavn-api-rs
[commit-activity]: https://img.shields.io/github/commit-activity/w/rajput-hemant/jiosaavn-api-rs
[forks]: https://img.shields.io/github/forks/rajput-hemant/jiosaavn-api-rs?style=flat
[stars]: https://img.shields.io/github/stars/rajput-hemant/jiosaavn-api-rs
