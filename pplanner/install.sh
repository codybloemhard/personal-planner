sudo mkdir -p /usr/local/bin
cargo build --release
sudo cp -f target/release/pplanner /usr/local/bin
sudo chmod 755 /usr/local/bin/pplanner
#sudo mkdir -p /usr/local/man
#sudo chmod 644 /usr/local/share/man/man1/st.1
