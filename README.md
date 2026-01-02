# Steveplays' portfolio

Made with [Yew](https://yew.rs) in [Rust](https://www.rust-lang.org), HTML, and CSS.

## Development

- Install [bacon](https://dystroy.org/bacon)
- Build the frontend
  - `trunk watch --config frontend/Trunk.toml`
- Start the backend
  - `bacon run -- -- -b backend/resources -f frontend/dist`
- View the website at <http://127.0.0.1:8000>
