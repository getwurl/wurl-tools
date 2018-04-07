# wurl-tools

```
$ wurl-tools --help
Automation for wurl

USAGE:
    wurl-tools [OPTIONS]

OPTIONS:
    -p, --ping <INSTRUCTION>     Send a ping control frame
    --pong <INSTRUCTION>         Send a pong control frame
    -m, --message <INSTRUCTION>  Send a WebSocket frame
    -c, --close <INSTRUCTION>    Send a close control frame

INSTRUCTION:
    An instruction is a command that defines when and what to send. For
    example, in order to send a custom ping every two seconds, the
    following instruction may be used:

    send {"type": "PING"} every 2s

    To close a connection after a set time, you can use this instruction
    with a close frame:

    send 1000 Goodbye! after 15s
```
