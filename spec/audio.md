# Audio

## *Type*: `Audio`

A buffer of floating-point audio.

### Fields

 - `size: val`
   - `frame: byte` - Number of channels per frame.
   - `chunk: byte` - Number of frames per chunk.
   - `count: half` - Number of chunks.
 - `addr: ptr[float]` - Interleaved samples; List of
   `size.frame * size.chunk * size.count` 32-bit floats.
 - `rate: int` - Sample rate of the audio (hertz)
 - `config: opt[Positions]` - Custom channel position configuration, default is
   FLAC/SMPTE/ITU-R recommendations.

### FLAC/SMPTE/ITU-R recommendations

 - 1 Channel: Mono (Mono)
   1. `(0, 0, 0, 0)`
 - 2 Channels: Stereo (Left, Right)
   1. `(-1, 0, 0, 0)`
   2. `(1, 0, 0, 0)`
 - 3 Channels: Surround 3.0 (Left, Right, Center)
   1. `(-1, 0, 0, 0)`
   2. `(1, 0, 0, 0)`
   3. `(0, 0, 0, 0)`
 - 4 Channels: Surround 4.0 (FrontL, FrontR, SurroundL, SurroundR)
   1. `(-0.5, 0, 0.8660254037844387, 0)`
   2. `(0.5, 0, 0.8660254037844387, 0)`
   4. `(-0.9396926207859084, 0, -0.3420201433256687, 0)`
   5. `(0.9396926207859084, 0, -0.3420201433256687, 0)`
 - 5 Channels: Surround 5.0 (FrontL, FrontR, Front, SurroundL, SurroundR)
   1. `(-0.5, 0, 0.8660254037844387, 0)`
   2. `(0.5, 0, 0.8660254037844387, 0)`
   3. `(0, 0, 1, 0)`
   4. `(-0.9396926207859084, 0, -0.3420201433256687, 0)`
   5. `(0.9396926207859084, 0, -0.3420201433256687, 0)`
 - 6 Channels: Surround 5.1 (FrontL, FrontR, Front, Lfe, SurroundL, SurroundR)
   1. `(-0.5, 0, 0.8660254037844387, 0)`
   2. `(0.5, 0, 0.8660254037844387, 0)`
   3. `(0, 0, 1, 0)`
   4. `(0, 0, 0, 1)`
   5. `(-0.9396926207859084, 0, -0.3420201433256687, 0)`
   6. `(0.9396926207859084, 0, -0.3420201433256687, 0)`
 - 7 Channels: Surround 6.1 (FrontL, FrontR, Front, Lfe, Back, Left, Right)
   1. `(-0.5, 0, 0.8660254037844387, 0)`
   2. `(0.5, 0, 0.8660254037844387, 0)`
   3. `(0, 0, 1, 0)`
   4. `(0, 0, 0, 1)`
   5. `(0, 0, -1, 0)`
   6. `(-1, 0, 0, 0)`
   7. `(1, 0, 0, 0)`
 - 8 Channels: Surround 7.1 (FrontL, FrontR, Front, Lfe, BackL, BackR, Left, Right)
   1. `(-0.5, 0, 0.8660254037844387, 0)`
   2. `(0.5, 0, 0.8660254037844387, 0)`
   3. `(0, 0, 1, 0)`
   4. `(0, 0, 0, 1)`
   5. `(-0.5, 0, -0.8660254037844387, 0)`
   6. `(0.5, 0, -0.8660254037844387, 0)`
   7. `(-1, 0, 0, 0)`
   8. `(1, 0, 0, 0)`
