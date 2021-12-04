# SHA1: Rust vs. OpenSSL

On an AMD Ryzen 7 3700X 8-Core Processor. Archlinux x86_64. Linux 5.15.5-arch1-1.

```
SHA1/sha-1 crate/0      time:   [67.277 ns 67.300 ns 67.320 ns]
SHA1/sha1 crate/0       time:   [174.90 ns 175.05 ns 175.23 ns]
SHA1/openssl sha1/0     time:   [87.448 ns 87.460 ns 87.474 ns]

SHA1/sha-1 crate/1      time:   [68.017 ns 68.061 ns 68.109 ns]
SHA1/sha1 crate/1       time:   [177.37 ns 177.52 ns 177.68 ns]
SHA1/openssl sha1/1     time:   [87.426 ns 87.445 ns 87.465 ns]
```
