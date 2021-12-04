# SHA1: Rust vs. OpenSSL

On an AMD Ryzen 7 3700X 8-Core Processor. Archlinux x86_64. Linux 5.15.5-arch1-1.

Short = 8 bytes, long = 64 bytes and huge = 512 bytes.

```
SHA1/sha-1 crate/short  time:   [68.501 ns 68.539 ns 68.588 ns]
SHA1/sha1 crate/short   time:   [193.22 ns 193.27 ns 193.35 ns]
SHA1/openssl sha1/short time:   [93.628 ns 93.640 ns 93.652 ns]

SHA1/sha-1 crate/long   time:   [91.868 ns 91.895 ns 91.925 ns]
SHA1/sha1 crate/long    time:   [273.37 ns 273.70 ns 274.08 ns]
SHA1/openssl sha1/long  time:   [118.38 ns 118.49 ns 118.59 ns]

SHA1/sha-1 crate/huge   time:   [292.09 ns 292.14 ns 292.18 ns]
SHA1/sha1 crate/huge    time:   [808.70 ns 809.03 ns 809.38 ns]
SHA1/openssl sha1/huge  time:   [317.79 ns 317.83 ns 317.87 ns]
```
