# Commands

Commands are data sent from the guest to the host.  Their structure is specific
to the channel which they are sent.  All commands have a lifetime, and the data
sent should only be freed after the command lifetime has ended.

## Command Lifetime Flow

 - Guest allocates a command on the heap
 - Guest sends the command address to the host with `ar()`
 - Host returns from `ar()` with the address of the command in the "ready list"
 - Host frees or reüses (for streams) the command as its lifetime has ended

## *Type*: `Command`

Commands are the way the Daku application sends messages to the environment.

### Fields

 - `chan: int` Channel to send data on
 - `addr: opt[T]` Pointer to command data for command `T`
