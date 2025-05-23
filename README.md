# Spotify Requests with Async Rust

Built a small tool with the crate reqwest that prints to the screen the names of the songs of the newly released albums in Spotify.

APIs:

- https://developer.spotify.com/documentation/web-api/reference/get-new-releases
- https://developer.spotify.com/documentation/web-api/reference/get-an-albums-tracks

### Motivation

Learn the basics of Rust async/await and the Rust toolchain.

### Development setup

Install pre-commit hooks:

```shell
pre-commit install
```

To run all pre-commit hooks without making a commit run:

```shell
pre-commit run --all-files
```

To create a commit without running the pre-commit hooks run:

```shell
git commit --no-verify
```


