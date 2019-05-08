sudo mkdir -p /usr/local/bin
cargo build --release
sudo cp -f target/release/pplanner /usr/local/bin
sudo chmod 755 /usr/local/bin/pplanner
sudo mkdir -p /usr/local/share/man/man1
sudo cp -f pplanner.groff /usr/local/share/man/man1/pplanner.1
sudo gzip /usr/local/share/man/man1/pplanner.1
