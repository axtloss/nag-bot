debug:
	cargo build

release:
	cargo build --release

install:
	mkdir /usr/bin
	cp target/release/crystal-nag-bot /usr/bin/nag-bot
