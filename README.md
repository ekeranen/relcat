# relcat (relative logcat)

An application for transforming [Android logcat](https://developer.android.com/studio/command-line/logcat)
logs to use relative timestamps.

Dual-licensed under Apache 2.0 or MIT.

# Build
This binary depends on [logcat](https://github.com/ekeranen/logcat).

1. `git clone https://github.com/ekeranen/logcat`
2. `git clone https://github.com/ekeranen/relcat`
3. `cd relcat`
4. `cargo build`

# Usage
```
adb logcat | relcat
```
