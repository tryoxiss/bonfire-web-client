# run: 
# `./SETUP_DEV.sh`
echo "installing trunk ..."
cargo install --locked trunk

echo "setting rust to nightly ..."
rustup default nightly

echo "adding wasm32-unknown-unknown build target ..."
rustup target add wasm32-unknown-unknown

echo "Compiling dependencies ..."
cargo run

echo "Don't worry if this last command errored! Its operating system dependent and it should work!"
echo ""
echo ""
echo "Now run `trunk serve` to serve the web page!"