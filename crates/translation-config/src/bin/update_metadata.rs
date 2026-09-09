//! Update various metadata from `translation-config.toml` files.
//!
//! Run with:
//! ```sh
//! cargo run --bin=update_metadata
//! ```

use translation_config::Config;

fn main() {
    let config = Config::load().unwrap();
    translation_config::update_metadata(&config);
}
