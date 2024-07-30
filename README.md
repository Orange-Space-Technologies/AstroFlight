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
make test
```
To run emulated tests on a x86 CPU, you need to have the `qemu-arm` userspace emulator installed, on Arch Linux, it's the `qemu-user` package.  
Not all tests can run, for example the sensors can't be initialized, so some tests will be skipped.  
Then, to run all runnable tests, just execute:  
```
make test-x86
```

### Software-in-the-loop simulation:
To run the software-in-the-loop simulation, you first have to have an exported simulation `csv` file from OpenRocket, it must have these **exact** values in it:  
```
Time (s),Altitude (m),Vertical velocity (m/s),Vertical acceleration (m/s²)
```

You then have to supply your exported file to AstroFlight, you can do that using the environment variable `FILENAME`.  
It's best to use absolute paths here.  

To then run the simulation on x86 (telemetry and other GPIO will be disabled), run:  
```
FILENAME="<path to your exported simulation csv file>" make sim-x86
```
Similary, to run the simulation on real hardware, run:  
```
FILENAME="<path to your exported simulation csv file>" make sim
```
This will allow you to test real GPIO outputs and telemetry. 