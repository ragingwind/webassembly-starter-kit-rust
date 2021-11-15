![logo](https://user-images.githubusercontent.com/124117/141869741-4f40b179-00c5-4d6c-8033-2e577016794b.jpg)

# WebAssembly Starter Kit with Rust

> Collective tools for learning WebAssembly with Rust. WARNING, Docker and packages are not working well on Apple M1

# Setup

```sh
docker build -t "wask/rust" -f Dockerfile .
```

# Run
```sh
docker run -t -i --mount type=bind,source="$PWD",target=/wask -p 3000:3000 "wask/rust"
```

# License

@ MIT
