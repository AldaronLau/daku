# 0x00 - Log

Log a message with an associated target and log level, usually to help with
debugging.

## Portal Channels

 0. Error Log Level
 1. Warn Log Level
 2. Info Log Level
 3. Debug Log Level
 4. Trace Log Level

## Readiness

Becomes ready once logging has completed (stopping the process after ready
wouldn't result in a partially-formed log message).

## *Command*: `Log`

If no target is necessary, prefer empty target for traditional stdout/stderr
compatibility.  Treat `I`/`D`/`T` as stdout, and `W`/`E`/`F` as stderr,
preferring `I` and `W`.

### Fields

 - `target: Text` Target name
 - `message: Text` Message to print

### Traps

 0. If `message` is not valid UTF-8, or contains a NUL byte
 1. If `target` is not valid UTF-8, or contains a NUL byte
 2. If address at `message.addr + message.size - 1` has no page
 3. If address at `target.addr + target.size - 1` has no page
