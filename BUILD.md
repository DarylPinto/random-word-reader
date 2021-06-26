## Automatic Builds
This repo is configured for automatic releases with compiled builds whenever version tags are pushed. Manually building for release is not required, but the steps are available below.

## Building for Release Manually

Prerequisits:
 - `rc.exe` from the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk)
 - `windres.exe` and `ar.exe` from [minGW64](http://mingw-w64.org)

```
cargo build --release
```

See https://github.com/mxre/winres for details
