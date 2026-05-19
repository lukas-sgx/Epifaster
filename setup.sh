mkdir -p $HOME/.local/bin/
cargo build -r
cp target/release/epiclang $HOME/.local/bin/epiclang
echo "Installation complete. Plugins have been copied to $HOME/.local/lib/epiclang/plugins/"