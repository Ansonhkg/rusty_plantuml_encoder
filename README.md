# PlantUML Encoder

A Rust library for encoding PlantUML diagram code, generating compressed strings for use in PlantUML image URLs.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
plantuml_encoder = "0.1.0"
```

## Usage

```rust
use plantuml_encoder::encodep;

fn main() {
    let diagram = "@startuml
Alice -> Bob: Authentication Request
Bob --> Alice: Authentication Response
@enduml";

    let encoded = encodep(diagram);
    println!("Encoded URL: http://www.plantuml.com/plantuml/img/{}", encoded);
}
```

## Features

- UTF-8 encoding
- DEFLATE compression
- Custom 64-character encoding for URL-safe strings

## Note

This encoder is specifically designed for PlantUML diagrams and may not be suitable for general-purpose encoding or encryption.
