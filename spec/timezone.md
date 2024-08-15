# TimeZone

## *Type*: `TimeZone`

A timezone specification

### Fields

 - `designation: TimeDesignation` Name of the timezone
 - `extension: opt[_]`: Reserved for timezone specification extensions
 - `offset: DateTime` Date and time of this timezone for UTC jan 1 00:00:00:000
   year 0
 - `adjustments: List[TimeAdjustment]` List of adjustments made in this timezone
 - `leap_seconds: List[LeapSecond]`: List of leap seconds
