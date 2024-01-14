set-up-for-the-first-time:
  cargo install trunk
  rustup target add wasm32-unknown-unknown

serve:
  trunk serve --open
