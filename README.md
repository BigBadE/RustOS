# Requirements:

Rust core library downloaded for your target (example for x86_64)

```rustup target add x86_64-unknown-none```

Nightly channel

```rustup override set nightly```

Install QEMU and set the environmental variable QEMU to the path to its
installation folder.

# TODO

- ~~Basic booting~~
- ~~Hardware interrupts~~
- ~~Heap allocator~~
- Async/Await
- File system
- Userspace programs
- Std
- Stack unwinding/debugging