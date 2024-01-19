# EnvVar

EnvVar is a Rust library for retrieving environment variables from different file types.

## Supported File Types

- Simple txt files (.txt, .env)
- JSON files (.json)

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
env_var = "0.1.0"
```

## Examples

### Examples with debug

```rust
use env_var::EnvHolder;
 // With debug flag set to true
let env_holder = EnvHolder::new(true);
let url = env_holder.get_var("url");
if let Some(url_value) = url {
    // Further processing
 }
```

### Examples without debug with and a custom file_name

```rust
use env_var::EnvHolder;
 // With debug flag set to true
let env_holder = EnvHolder::new(false).with_file_name("custom_file.env");
let url = env_holder.get_var("url");
if let Some(url_value) = url {
    // Further processing
 }
```
