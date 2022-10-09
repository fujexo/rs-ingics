# Ingics

![Build and Tests](https://github.com/fujexo/rs-ingics/actions/workflows/test.yml/badge.svg)

A rust library to parse the manufacturer data of [Ingics BLE Sensors](https://www.ingics.com/tag.html).

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
ingics = "0.1.1"
```

### Examples

Currently, one example is available: `ble_influx`. The example reads sensors via BLE and writes the data to an Influx DB.
To run it, clone this repository and execute:
```bash
cargo r --example ble_influx --features influxdb
```

## Crate Features

The following features can be optionally enabled:

* `influxdb` enables influxdb support
