# JADUL DASH
A just-for-fun project to replace the aging head unit in my car with something I built myself.

## The story

My car still has its original head unit: a small screen, a clunky interface, and no way to use
the apps I rely on every day, like maps and music from my phone. Buying an aftermarket unit would
be the sensible fix. But I wanted to know how hard it would be to build one myself, so I'm doing
that instead.

The idea is simple: put a small Linux computer and a touchscreen behind the dashboard. Then run a
lightweight dashboard app that can act as an **Android Auto** receiver. When I plug my phone into
the USB port, the app should pick it up and show Android Auto (Google Maps, media, and so on) on
the car screen, just like a factory unit would.

This is a hobby project: no deadlines, no guarantees, and plenty of experimenting along the way.

## What it's built with

- **[Rust](https://www.rust-lang.org/):** fast, memory-safe, and fun to write.
- **[Slint](https://slint.dev/):** a declarative UI toolkit well suited to embedded and
  touchscreen interfaces. The dashboard layout lives in `ui/dashboard.slint`.
- **[Tokio](https://tokio.rs/):** an async runtime for background work, like waiting for a phone
  to connect.
- **[android-auto](https://crates.io/crates/android-auto):** a crate that handles the Android
  Auto protocol over USB (AOA).

## Current status

Early days. What exists right now:

- A 1024×600 dashboard window with a side menu (**Home / Menu** and **Android Auto** buttons) and
  a main area where the Android Auto or navigation view will be drawn.
- A background task that will listen for an Android phone on the USB port.
- A first sketch of the Android Auto session handler (`src/android_service.rs`) with callbacks
  for connect, disconnect, and incoming video packets. It isn't hooked up yet.

Next steps:

- [ ] Connect the Android Auto USB session to the background worker
- [ ] Decode the video stream and show it in the main area
- [ ] Send touch input from the screen back to the phone
- [ ] Play audio from the phone through the car speakers
- [ ] Get it running on real hardware inside the car

## Project layout

```
.
├── build.rs                 # Compiles the Slint UI at build time
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point: starts the UI and the USB worker
│   └── android_service.rs   # Android Auto session handler (work in progress)
└── ui/
    └── dashboard.slint      # Dashboard layout
```

## Building and running

You'll need a recent Rust toolchain (the project uses the 2024 edition) and, on Linux, a few
system libraries for Slint's windowing and font support. On Ubuntu/Debian:

```sh
sudo apt install -y pkg-config libfontconfig1-dev libxkbcommon-dev libwayland-dev \
    libx11-dev libxcursor-dev libxrandr-dev libxi-dev libgl1-mesa-dev
```

Then:

```sh
cargo run
```

A window titled **Car Dashboard - Rust** should open, waiting for a phone to connect.

## Disclaimer

This is a personal side project, not a certified automotive product. If you try anything
similar, keep driving safety first: don't fiddle with the screen while driving, and be careful
with your car's electrical system.
