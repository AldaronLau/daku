# 0x03 - Serve 🧪

Serve resources over HTTPS.

SSE is expected to be implemented as an abstraction over this API (rather than
be provided as its own portal).

## Portal Channels

 0. Accept HTTPS clients

## Connection

Client device channel is opened upon readiness.  The channel allocation should
happen immediately upon return of `ar()`, and must follow in the order of the
ready list.

## Readiness

Becomes ready once either
 - A client connects.
 - The number of connection errors (with `errors` being non-null) since the last
   successful connetion, in one of the 4 categories has exceeded 255

## *Command*: `Serve`

### Fields

 - `config: val`
   - `port: half` Port number to connect to
   - `connections: half` How many client connections to attempt maximum
     - If sign bit is set to negative, allow other computers to connection as a
       client
 - `errors: opt[val]` Output number of errors that occured since last successful
   connection (or since start, if becoming ready on first success)
   - `load: byte` Number rejected based on load being too high (clients
     connecting is larger than ready list size)
   - `maximum: byte` Number rejected based on too many connections exceeding
     maximum set with `connections`
   - `https: byte` Number rejected for invalid HTTPS
   - `timeout: byte` Number rejected based on timeout on sending headers

### Traps

 0. If `connections = -32768`
 1. If `port = 0` (local only) and `connections` sign bit is negative
 2. If address at `errors + 3` has no page
