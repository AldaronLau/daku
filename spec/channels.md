# Channels

## Channel 0

Channel 0 is a special channel that allows you to send arbitrary bytes
([`Buffer`](./buffer.html)) to the host from the guest.  This is what a custom
plugin API would use.  Sending commands on channel 0 may also open "Device
Channels".

## Portal Channels

Portal channels represent a bus (and permission to talk to it) on the computer.
Portal channels are opened before the program starts (based on the `daku` custom
section).  Going in order of the portals, will allocate channels in ascending
order starting from 1.  There may be more than one channel per portal.  They can
not and will not be closed until the WASM module exits.  

## Device Channels

Device channels represent something (physically or virtually) connected to a
bus.  Device channels are opened by sending commands to portals (this command
is not allowed to fail if constructed properly, even if the data is stale - so
you may open a channel to a disconnected device).  They are allocated after the
portal channels.  When the device channel is closed it's number goes into a
garbage list.  The last ID is popped off the garbage list when opening a new
device channel (if it exists).
