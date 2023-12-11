# Communication Protocol

## Message Structure
as usual, the message structure follows like this:
`<kind><data-length><data>`

- the `cmd` part is a single-byte which represents what kind of message is this.
- the `data-length` part is an unsigned 4-byte integer in big-endian which represents the length of the data part.
- the `data` part is an arbitary length data (length specified in `data-length`) this segmend contains data and/or attributes which are individual.

## Client -> Server messages

### JOIN (0x6A)
as the client connects to the server, the server shall expect this message, containing the name of the player as an attribute (ascii)

`data` part structure:

    `<utf8-encoded name of the player joining>`

### LOG (0x6C)
logs a message to the server

`data part structure`

    `<utf8 encoded log content>`

## Server -> Client messages

### JOIN_OK (0x4A)
as the server recieves the *JOIN* message, it should send back this one. it has no content whatsoever but it tells the client that he joined successfully.
