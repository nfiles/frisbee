# frisbee
simple static file server

```
frisbee 0.1.0
Nathan Files <nathanwfiles@gmail.com>

USAGE:
    frisbee [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -p, --public <public>    Path prefix for HTTP requests [default: /]
    -r, --root <root>        Directory to serve from disk [default: .]
    -V, --version            Print version information

Port/Address/etc can be configured with Rocket's native configuration system
https://rocket.rs/v0.5-rc/guide/configuration/

ROCKET_PORT=80 frisbee
```

# Install

```sh
cargo install frisbee
OR
cargo install --git https://github.com/nfiles/frisbee.git --branch main
```

# Publishing

```sh
# Publish the crate
cargo release <level>

# push changes
git push
git push --tags
```
