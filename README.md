# wurl-tools [![Build Status](https://travis-ci.org/getwurl/wurl-tools.svg?branch=master)](https://travis-ci.org/getwurl/wurl-tools) [<img src="https://github.com/getwurl/wurl/raw/master/assets/logo.png" width="280" align="right" alt="wurl">](https://github.com/getwurl/wurl)

> Plug and play automation for [wurl][wurl]

[wurl][wurl] was always designed to be composable. This means other applications
can send messages by piping into it with a unix pipe (`|`). A common use for
this is to implement a ping feature, where a ping message is periodically
sent to the server as a keepalive implementation. `wurl-tools` makes this work
out of the box by passing an option:

    wurl-tools -p "every 15s"

All data that `wurl-tools` recieves on `stdin` is forwarded to `stdout` to make
further pipelining possible. This means that any data piped through `wurl-tools`
is forwarded to [wurl][wurl], which will send the data to the server.



## Usage

```
wurl-auth

USAGE:
    wurl-tools --close <INSTRUCTION>... --message <INSTRUCTION>... --ping <INSTRUCTION>... --pong <INSTRUCTION>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --close <INSTRUCTION>...      Send a close control frame
    -m, --message <INSTRUCTION>...    Send a WebSocket frame
    -p, --ping <INSTRUCTION>...       Send a ping control frame
        --pong <INSTRUCTION>...       Send a pong control frame

INSTRUCTION:
    An instruction is a command that defines when and what to send. For
    example, in order to send a custom ping every two seconds, the
    following instruction may be used:

    send {"type": "PING"} every 2s

    To close a connection after a set time, you can use this instruction
    with a close frame:

    send 1000 Goodbye! after 15s
```

## Supported units of time

- `ms` - Milliseconds
- `s`, `sec` - Seconds
- `m`, `min`  - Minutes
- `h`  - Hours
- `d`, `day`, `days`  - Days

## Example

```
wurl-tools -p 'every 15s' | wurl wss://websocket.example.com
```

## Install

### [crates.io][crates.io]

> NOT YET PUBLISHED

    $ cargo install wurl-tools

If you are a rust programmer, it is easy to install wurl using the cargo CLI.

To update to a newer version, simply overwrite the old executable when
installing.

    $ cargo install --force wurl-tools

### [binaries][binaries]

All tagged releases on GitHub have compiled binaries attached to them.

### [docker][docker]

    $ docker run -it --rm esphen/wurl-tools

If you do not have the rust toolchain installed and there is no package for your
OS, you may use the docker image, which will run on all platforms that docker
supports.

If you use docker, it may be an idea to add an alias to make running it through
docker easy. For example:

    $ alias wurl="docker run -it --rm esphen/wurl-tools"

[wurl]: https://github.com/getwurl/wurl
[crates.io]: https://crates.io
[binaries]: https://github.com/getwurl/wurl-tools/releases
[docker]: https://store.docker.com/community/images/getwurl/wurl-tools

