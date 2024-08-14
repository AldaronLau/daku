# Guest Exports

 - Export 32-bit memory as `mem`
 - Export "main" function as `run`
 - Export ready list global pointer as `rl4` for a size 16 ready list
   - `rl0` for a size 1 ready list
   - `rl1` for a size 2 ready list
   - `rl2` for a size 4 ready list
   - `rl3` for a size 8 ready list
   - `rl4` for a size 16 ready list
   - `rl5` for a size 32 ready list
   - `rl6` for a size 64 ready list
   - `rl7` for a size 128 ready list
   - `rl8` for a size 256 ready list
   - `rl9` for a size 512 ready list

## *Type*: `ReadyList[N]`

The list of ready/complete commands

### Fields

 - `ready_list: [opt[T]; N]` Ready list of commands
