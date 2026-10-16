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
to **TrouBLE A**, read Battery Level (42), and wait for `security complete` with
`new_bond=true, bonded_peer=true`.
Keep A connected, then connect another client to **TrouBLE B** and read its Battery Level.
Pair A before connecting B: the security manager supports one pairing at a time.
Both connections must retain their addresses while advertising set 0 is reused.
The connection logs show A at `C0:05:04:03:02:01` and B at `C0:05:04:03:02:02`.
Each successful Battery Level read is logged with its connection name; also check
that the client receives 42. Disconnect A and read B again, then disconnect B.
Reconnect A and B without resetting the DK. Each should report `known_bond=true`,
then `link encrypted` with `bonded_peer=true`, without creating a new bond.
`new_bond` describes the security event's payload, not whether an older bond exists.
Bonds are kept in RAM; after resetting the DK, remove the clients' old bonds before
pairing again. Omit `security` to test address tracking without pairing.
