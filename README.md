# SHA1: Rust vs. OpenSSL

On an AMD Ryzen 7 3700X 8-Core Processor. Archlinux x86_64. Linux 5.15.5-arch1-1.

```
SHA1/Rust SHA1/0        time:   [67.242 ns 67.273 ns 67.300 ns]
SHA1/OpenSSL SHA1/0     time:   [88.140 ns 88.172 ns 88.206 ns]
SHA1/Rust SHA1/1        time:   [68.137 ns 68.159 ns 68.182 ns]
SHA1/OpenSSL SHA1/1     time:   [87.181 ns 87.231 ns 87.289 ns]
```
