# chkstk_stub

A minimal Rust build helper crate that provides a stub implementation of the macOS/iOS runtime function `__chkstk_darwin`.

## Why this crate exists

When building C/C++ code for **iOS** or **macOS** using Rust, you may encounter linker errors like:

```csharp
Undefined symbols for architecture arm64:
  "___chkstk_darwin", referenced from:
      ...
```

This happens when your code (or a dependency) uses large stack allocations, and the platform would normally insert a call to `__chkstk_darwin` — but the symbol is missing in your build environment.
On some targets, especially when cross-compiling static libraries, the function isn’t automatically provided.

`chkstk_stub` solves this by compiling and linking a no-op version of the function:

```c
void __chkstk_darwin(void) {}
```

This is **safe** if:

You control all code that might hit this call site.

You are certain that large stack allocations are safe in your environment.

You just need to bypass the missing symbol to get a successful build.

## How it works

The `build.rs` file in this crate:

1. Writes `chkstk_stub.c` into the Cargo build output directory (`OUT_DIR`).

2. Compiles it into a static library using the `cc` crate.

3. Ensures Cargo links the resulting `libchkstk_stub.a` into your build.

## Usage

```toml
[build-dependencies]
chkstk_stub = { version = "0.1.0" }
```

## Notes & Warnings

- This is **not a full implementation** of `__chkstk_darwin` — it’s a no-op.

- In most cases, stack probing is only needed for large stack frames; skipping it may be safe for your use case, but **you should verify**.

- This is intended for **linker unblocking** in controlled build environments, not as a general-purpose solution for production code.
