# Bluestacks adb

Created a program to auto connect to the ADB port of Bluestacks using Rust

Run the command to create the exe

```shell
cargo build --release
```

And finally add it to the path

Or to the powershell using this

```shell
code $profile
```

inside the ps1 file, add the following

```ps1
Set-Alias -Name bluestacks-adb -Value "<path-to-the-rust-project>\target\release\bluestacks-adb.exe"
```
