# SH-OS
Simple OS project for practicing Rust (nightly) &amp; x86-64 Assembly programming, &amp; learning more about computers (hardware &amp; software) - This project is a work in progress, currently only displays a message when built/ran.

To build & run SH-OS:
- Install QEMU (Emulator) & ensure it's in your path (you'll need to install MSYS2 for this): https://www.qemu.org/download/ 
- Change your directory or local machine to use Rust nightly:
-     Run in the root directory: rustup default nightly
- Install Rust's LLVM tools - Run: rustup component add llvm-tools-preview
- Install the Rust BootImage tool: cargo install bootimage
- Clone the repository: git clone [https://github.com/your-username/sh-os.git](https://github.com/your-username/sh-os.git)
- Navigate to the root directory: cd sh-os
- Build the kernal: cargo build
- Run in QEMU: cargo run

Success! Now you'll see the message: Hello Sh-OS!
                                     Everything is working!
