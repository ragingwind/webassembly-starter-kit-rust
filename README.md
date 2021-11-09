# WebAssembly Starter Kit with Rust

> Collective tools for learning WebAssembly with Rust

# Setup

```sh
# build docker
docker build -t "wsk/rust" -f Dockerfile .
```

# Run
```sh
# run docker with 
# - tty interactive
# - mount src to
# - mapping inner port to outer
docker run -t -i --mount type=bind,source="$PWD/src",target=/src -p 8080:8080 "wsk/rust"
```

# License

@ MIT