# AstroFlight

AstroFlight is a rocket flight software developed by Orange Space Technologies for the AstroCore family of flight computers

## Development env setup
You need to have the `arm-unknown-linux-gnueabihf` target installed:  
```
rustup add target arm-unknown-linux-gnueabihf
```

### You also need the RPi linker:
Clone the compiler toolchain (you can choose any folder, just remember to replace the path everywhere):  
```
git clone https://github.com/raspberrypi/tools $HOME/rpi_tools
```
Edit the `~/.cargo/config` file using your favourite text editor:  
```
nano ~/.cargo/config
vim ~/.cargo/config
```
Tell Cargo to use a specific linker version for your target:  
```
# in ~/.cargo/config

[target.arm-unknown-linux-gnueabihf]
linker = "rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
```

## Compiling and running tests
We use `make` to make some commands easier, if you don't want to use `make`, take a look inside the `Makefile` file for the commands.  
### Compiling:
To compile a release version:  
```
make build
```
To compile a debug build:  
```
make build-debug
```
To start a release build:  
```
make run
```
### Testing:
To run tests on the real hardware (Raspberry Pi Zero 2W):  
```
make run
```
To run emulated tests on a x86 CPU, you need to have the `qemu-arm` userspace emulator installed, on Arch Linux, it's the `qemu-user` package.  
Not all tests can run, for example the sensors can't be initialized, so some tests will be skipped.  
Then, to run the tests, just execute:  
```
make test-x86
```
