all: run

data:
	mkdir data


build:
	cargo build --release

build-debug:
	cargo build

run-release:
	cargo build --release

run: build data
	target/arm-unknown-linux-gnueabihf/release/astro_flight

run-qemu: data
	RUSTFLAGS='-C target-feature=+crt-static' cargo --config 'runner="qemu-arm"' run

only-run:
	target/arm-unknown-linux-gnueabihf/release/astro_flight

test-x86:
	RUSTFLAGS='-C target-feature=+crt-static' cargo --config 'runner="qemu-arm"' test

test-arm:
	cargo test

install-service:
	ln -s /home/pi/AstroFlight/astroflight.service /etc/systemd/system/astroflight.service
	systemctl daemon-reload

enable: install-service
	systemctl enable astroflight.service

disable: install-service
	systemctl enable astroflight.service

start: install-service
	systemctl start astroflight.service
