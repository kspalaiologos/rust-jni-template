# rust-jni-template
A template for developing JNI libraries using the Rust programming language

## Operating systems and architectures supported by the CI

Currently:
- x86_64 Linux
- ARM64 Linux
- s390x Linux
- i686 Linux
- armhf Linux
- Mips64 Linux
- Riscv64 Linux
- PowerPC64 Linux
- PowerPC64LE Linux
- x86_64 Windows
- i686 Windows

## FAQ

- Why do you not support MacOS?

Apple's predatory policies state that it is illegal to emulate MacOS in order to build software for Mac if the host is not Apple's hardware, hence support for MacOS will probably not be added, unless a GitHub Action for emulating ARM M1 MacOS inside the Intel runners provided by GitHub becomes available and someone submits a pull request to the repository.

- Why do you not support more architectures?

PRs welcome.

- What about musl support? Shouldn't the .so files be linked statically?

PRs welcome.
