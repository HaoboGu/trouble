# nRF examples

## Setup

These examples are set up assuming you are using [probe-rs](https://probe.rs) for flashing and debugging your chips.  Make sure you have it with:

```bash
cargo install cargo-binstall # binary installer tool (optional but recommended)
cargo binstall probe-rs-tools # or cargo install if you don't use binstall
```

You may also need to install the chip's toolchain too, if so that would be done with, for example:

```bash
rustup target add thumbv8m.main-none-eabihf
```

Build dependencies (nrf-sdc) also require clang to compile:

```bash
# Linux
sudo apt install llvm
# Windows (multiple ways to install)
choco install llvm 
# Mac
brew install llvm
```

We use features to turn on the appropriate configurations for the chip you are flashing to.  Currently supported chips are:

- `nrf54l15`
- `nrf54lm20` (nRF54LM20A)

## Run

To build and run an example on your device, plug it in and run i.e.:

```bash
cd examples/nrf54 # make sure you are in the right directory

cargo run --release --features nrf54l15 --target thumbv8m.main-none-eabihf --bin ble_bas_peripheral
```

## Per-connection advertising addresses on nRF54LM20A DK

Connect the DK's debugger USB port and run from `examples/nrf54`:

```bash
cargo run --release --features nrf54lm20,security --bin ble_advertise_multiple
```

The runner targets `nRF54LM20A`; no nRF52840 board is needed. Connect one BLE client
to **TrouBLE A**, read Battery Level (42), and wait for its pairing/encryption logs.
Keep A connected, then connect another client to **TrouBLE B** and read its Battery Level.
Both connections must retain their addresses while advertising set 0 is reused.
Disconnect both, then repeat. This example does not persist bonds; remove old client
bonds before retrying. Omit `security` to test address tracking without pairing.
