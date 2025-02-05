# Simple Benchmark

## macOS

| tool           | user | sys | total | CPU% | rate       | ratio |
|:--------------:|:----:|:---:|:-----:|:----:| ----------:|:-----:|
| native         |  1.1 | 1.5 |  4.8  |  53% | 3,585 MB/s | 14.1x |
| wasi/wazero O2 |  4.2 | 1.0 |  6.6  |  78% | 2,645 MB/s | 10.4x |
| wasi/wazero Os |  4.2 | 1.4 |  7.5  |  74% | 2,302 MB/s |  9.1x |
| (fgrep)        | 67.0 | 0.8 | 68.3  |  99% |   254 MB/s | (1x)  |
| wasi/wasmtime  | 35.7 | 64  | 99.2  | 101% |   175 MB/s |  0.7x |
