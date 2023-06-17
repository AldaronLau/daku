# 0x02 - Fetch 🧪

Do an HTTPS request to specified URL.

SSE is expected to be implemented as an abstraction over this API.

## Connection

Host device channel is opened upon readiness (allocated only if successful).
The channel allocation should happen immediately upon return of `ar()`, and must
follow in the order of the ready list.

## Readiness

Becomes ready once either
 - The resource host can't be reached (network error), `capacity` set to 0.
 - The resource host hung up, `capacity` set to 0.
 - The resource host has sent back a chunk of the resource (a new device
    channel has been opened)
   1. Buffer is not big enough, `buffer` overwritten up to `capacity` bytes, and
      `capacity` modified how many more bytes are required
   2. Buffer is big enough, `buffer` overwritten, `capacity` unchanged.

## *Command*: `Fetch`

### Fields

 - `url: Text` - URL to do an HTTPS request to (does not include `https://`
   protocol)
 - `headers: Text` - Newline-delimited extra headers to send
 - `body: opt[List[byte]]` - Optional payload/content body to send
 - `method: int` - 0: GET, 1: HEAD, 2: POST, 3: PUT, 4: DELETE.
 - `capacity: ptr[int]` - (In/Out) Pointer to capacity of `buffer`.
 - `buffer: ptr[List[byte]]` - (In/Out) Pointer to buffer for receiving parts of
   the HTTP response.

### Traps

 - If `url` does not start with one of
   - Domain name lowercase (ranged a~z or 0~9, `-` and `.` allowed after first
     character, but not consecutively, and not last character before termination
     character - one of `:/?`)
   - IPv4 Address - 4 integers ranged 0:255 each separated by `.`)
   - Ipv6 Address - `[`, then 8 lowercase hexaxecimal numbers from 1 to 4 digits
     separated by `:`, and `]`.  A grouping of consecutive zero numbers can be
     replaced with `::`, rather than requiring all 8 numbers.
 - If `url` after domain/IP doesn't either start with one of `:/?` or end
 - If `url` sections `:/?` are out of order
 - If `url` port after `:` is not in range 0~65535
 - If `url` path after `/` is not `a~z`, `A~Z`, `-`, `_`, `%`, `.`, `~`, `+`,
   or `/`
 - If `url` query after `?` is not `a~z`, `A~Z`, `0~9`, `-`, `_` `.`, `~`, `+`,
   `=`, `;`, or `&`
 - If `headers` begins with `\n` or ends with `\n`
 - If `headers` line does not match `Title-Kebab-Case: expected-type`
 - If `headers` line contains an invalid (Like `Not-A-Header`), redudant (Like
   `Content-Length`), or insecure (Also like `Content-Length`) `header`
 - If address at `body.addr + body.size` has no page
 - If input `capacity` is less than `buffer.size`
 - If address at (input) `buffer.addr + body.capacity` has no page
