# Surge XT <sup>for Rust!</sup>
### Welcome! These are Rust bindings for the Surge XT synthesizer.
[Surge XT](https://surge-synthesizer.github.io) is a free and open-source hybrid synthesizer. Originally written and sold commercially by @kurasu (Claes Johanson) at [Vember Audio](https://vemberaudio.se); then open-sourced in September 2018 and maintained by a team of developers since.

## Types.
This crate provides bindings for both **low-level** and **high-level** control of the synthesizer.

As of the time of writing, both types of bindings are focused on the *synthesis itself*. This means components that interface with other parts of the Surge XT project (such as `PatchDB`) are not implemented.

Regardless, chances are you can do everything with the synthesizer itself, as it ends up exposing its fair share of helper functions.

### High-level (EasySurge).
The high-level bindings for the synthesizer can be accessed through the struct `EasySurge`. It's called like that because it's easy to use.

Usage of these bindings is heavily based on the visual interface of the synthesizer. If you're configuring patches through the API, be sure to keep an instance of Surge XT open to check parameter names.

```rust
// example code to print a bunch of unreadable FM wave data.
use surge_rs::EasySurge;

fn main() {
    let mut synth = EasySurge::new(48000.0);
    synth.set_parameter("A Osc 1 Type",             0.45).unwrap();
    synth.set_parameter("A Osc 1 M1 Amount",        0.5).unwrap();
    synth.set_parameter("A Osc 1 M1 Ratio",         0.62).unwrap();
    synth.set_parameter("A Amp EG Attack",          0.25).unwrap();
    synth.set_parameter("A Amp EG Decay",           0.15).unwrap();
    synth.set_parameter("A Amp EG Sustain",         0.5).unwrap();
    synth.set_parameter("A Amp EG Release",         0.5).unwrap();

    synth.play_note(0, 60, 127, 0, 0, 0);

    // processing happens 32 samples at a time.
    // i only take the left samples (i'm left-handed).
    let mut samples = vec![0.0; 32*256];
    for _ in 0..32 {
        synth.process();
        samples.extend_from_slice(&synth.pull_buffer()[0]);
    }

    // your terminal might break if you don't sync outputs (eprint).
    for (i, s) in samples.into_iter().enumerate() {
        eprintln!("N: {}\tV: {}", i, s);
    }
}
```
Make sure to also pull in the `EasySurgeError` enumeration to handle errors!

### Low-level (glue).
The low-level bindings (FFI) for the synthesizer can be accessed through the module `glue`. It's called like that because it is the glue I used to create the rest of the library.

Usage of these bindings follows the synthesizer's internal C++ engine. In other words, they're FFI bindings with some adjustments to make them fit Rust more (like replacing argument defaults with `Option`s).
```rust
// example code to print a bunch of unreadable FM wave data.
use surge_rs::glue::synthesizer::{SurgeSynthesizer, SurgeId};

fn main() {
    let mut synth = SurgeSynthesizer::new(48000.0);

    // make sure you keep a list because you'll start needing it pretty soon.
    let changes = [
        (222, 0.45),
        (225, 0.5),
        (226, 0.62),
        (320, 0.25),
        (322, 0.15),
        (324, 0.5),
        (325, 0.5),
    ];

    for (i, v) in changes {
        let mut id = SurgeId::empty();

        // first we get the id from the index.
        if !synth.from_synth_side_id(i, &mut id) { panic!("idx {} get error.", i); }
        synth.set_parameter01(&mut id, v, None, None);
    }

    synth.play_note(0, 60, 127, 0, 0, 0);

    // processing happens 32 samples at a time.
    // i only take the left samples (i'm left-handed).
    let mut samples = vec![0.0; 32*256];
    for _ in 0..32 {
        synth.process();
        samples.extend_from_slice(&synth.pull_buffer()[0]);
    }

    // your terminal might break if you don't sync outputs (eprint).
    for (i, s) in samples.into_iter().enumerate() {
        eprintln!("N: {}\tV: {}", i, s);
    }
}
```
This might look simple. It's not.

You should only really use `glue` if you need to probe deeper into the synthesizer and have no way to do so through `EasySurge`.
