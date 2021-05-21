编辑 `~/.cargo/config`

加上

```
[target.aarch64-linux-android]
linker = "aarch64-linux-android30-clang"
ar = "aarch64-linux-android-ar"

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```


windows是避免缺少dll的报错，参见

https://rust-lang.github.io/rfcs/1721-crt-static.html


rust 宏系统

https://hardocs.com/d/rustprimer/macro/macro.html
