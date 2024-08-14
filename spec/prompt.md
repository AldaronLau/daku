# 0x01 - Prompt

Receive a line of textual user input from a debugging console.

The text appended to the buffer won't contain the newline character.

## Portal Channels

 0. Developer prompt

## Readiness

Becomes ready once a line of text has been entered.  With either

 1. Buffer is not big enough, with `capacity` modified to what is required
 2. Buffer is big enough, `command` text overwritten

## *Command*: `Prompt`

Read textual user input from some source.

### Fields

 - `capacity: ptr[int]` - (In/Out) Pointer to capacity of `command`.
 - `command: ptr[Text]` - (In/Out) Pointer to user-sent line of input.

### Traps

 0. If input `capacity` is less than `command.size`
 1. If address at (input) `command.addr + capacity` has no page
 2. If address at `capacity + 3` has no page
