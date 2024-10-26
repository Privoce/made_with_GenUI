# Build Exe with Icon

## add `embed-resource` as build-dependencies

```toml
[package]
# ...
build = "build.rs"

# ...
[build-dependencies]
embed-resource = "2.5"
```

## add build.rs

### Easy

```rust
extern crate embed_resource;

fn main() {
    embed_resource::compile("app_icon.rc", embed_resource::NONE);
}
```

### Set different icon in different compile type
```rust
fn main() {
    if cfg!(target_os = "windows") {
        let icon_file = if cfg!(debug_assertions) {
            "debug_icon.rc" // --debug
        } else {
            "release_icon.rc" // --release
        };
        embed_resource::compile(icon_file, embed_resource::NONE);
    }
}
```

## add app_icon.rc (debug_icon.rc/release_icon.rc)

```rc
#pragma code_page(65001)
1 ICON path/to/icon.ico
```

## prepare an ico file

you need to prepare a ico file

- 16 * 16
- 32 * 32
- 48 * 48
- 64 * 64
- 128 * 128
- ...