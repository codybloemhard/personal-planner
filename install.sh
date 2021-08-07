#!/bin/sh
# install binairy
sudo mkdir -p /usr/local/bin
cargo build --release
sudo cp -f target/release/pplanner /usr/local/bin
sudo chmod 755 /usr/local/bin/pplanner
# install other files
sudo mkdir -p ~/.config/pplanner
sudo rm -rf ~/.config/pplanner/help
sudo cp -rf help ~/.config/pplanner/help
sudo cp LICENSE ~/.config/pplanner/LICENSE
# install manpage
sudo mkdir -p /usr/local/share/man/man1
sudo cp -f pplanner.ms /usr/local/share/man/man1/pplanner.1
sudo gzip /usr/local/share/man/man1/pplanner.1
