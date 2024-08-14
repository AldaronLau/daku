# Introduction

Daku is an asynchronous host interface abstraction API for WebAssembly plugins,
drivers, applications, and more!  Daku is both an API and a file format.  Ardaku
is an engine you can use for running Daku modules on different Wasm runtimes.

## Daku Specification v1.0.0-pre.0 (draft v12)

The current version of Daku targets the full WebAssembly 2.0 spec without any
non-standard or experimental features.

## Goals

 - Simple
 - Efficient
 - Modular
 - Minimal (in API surface, and memory footprint)
 - Asynchronous
 - Backwards Compatible
 - Reduced context switching
 - Security-first
 - First-class multimedia portals (WASI compatible)
