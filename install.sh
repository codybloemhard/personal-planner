#!/bin/sh
# install binairy
doas mkdir -p /usr/local/bin
cargo build --release
doas cp -f target/release/pplanner /usr/local/bin
doas chmod 755 /usr/local/bin/pplanner
# install other files
doas mkdir -p ~/.config/pplanner
doas rm -rf ~/.config/pplanner/help
doas cp -rf help ~/.config/pplanner/help
doas cp LICENSE ~/.config/pplanner/LICENSE
# install manpage
doas mkdir -p /usr/local/share/man/man1
doas cp -f pplanner.ms /usr/local/share/man/man1/pplanner.1
doas gzip /usr/local/share/man/man1/pplanner.1
