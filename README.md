# EU & UK VAT Validator & Rate Engine — Rust Client Crate

[![Crates.io](https://img.shields.io/crates/v/stanzaapi-vat-validator.svg)](https://crates.io/crates/stanzaapi-vat-validator)
[![Documentation](https://docs.rs/stanzaapi-vat-validator/badge.svg)](https://docs.rs/stanzaapi-vat-validator)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Stanza API](https://img.shields.io/badge/Powered%20by-Stanza-blue)](https://stanzaapi.com)

> Deterministic offline checksum validator for EU-27 and UK VAT numbers with instant 2026 standard & reduced tax rate lookups.

Official high-performance, asynchronous Rust client library for **EU & UK VAT Validator & Rate Engine**, built on the [Stanza Micro-API Network](https://stanzaapi.com). Uses pure Rustls TLS (zero C/OpenSSL dependencies) and Tokio for maximum concurrency and safety.

* 🌐 **Online Interactive Sandbox:** [Test your inputs live](https://stanzaapi.com/tools/vat-validator)
* 📚 **API Reference & Schemas:** [View documentation on Stanza](https://stanzaapi.com/tools/vat-validator)
* ⚡ **Platform Overview:** [Explore the Stanza Developer Network](https://stanzaapi.com)

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
stanzaapi-vat-validator = "1.0.0"
tokio = { version = "1.0", features = ["full"] }
```

Or use `cargo add`:

```bash
cargo add stanzaapi-vat-validator
```

---

## 🚀 Quickstart

```rust
use stanzaapi_vat_validator::VatValidatorClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reads STANZA_API_KEY from environment automatically
    // Public edge default: https://api.stanzaapi.com/vat-validator
    let client = VatValidatorClient::new(None, None);

    let response = client.validate("DE123456789").await?;

    if response.success {
        println!("Verification Success: {:?}", response.data);
    } else {
        eprintln!("Validation Error: {:?}", response.error);
    }

    Ok(())
}
```

---

## 📄 Example Response

```json
{
  "success": true,
  "data": {
    "valid": true,
    "vat_number": "DE123456789",
    "country_code": "DE",
    "standard_rate": 19,
    "reduced_rate": 7
  }
}
```

---

## 🔗 Useful Links

* [EU & UK VAT Validator & Rate Engine Interactive Sandbox](https://stanzaapi.com/tools/vat-validator)
* [Stanza Developer Directory](https://stanzaapi.com)
* [Source Code & Issue Tracker](https://github.com/StanzaAPI/vat-validator-rust)

## 📄 License

MIT © Stanza — Powered by [Stanza](https://stanzaapi.com).
