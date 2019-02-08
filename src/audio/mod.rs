use alto::{Alto, AltoResult, DeviceObject, efx, Mono, Stereo, Source};
use lewton::{OggReadError, VorbisError};

use log::info;

pub fn run() -> AltoResult<()> {
    let alto = Alto::load_default()?;

    for s in alto.enumerate_outputs() {
        info!("Found device: {}", s.to_str().unwrap());
    }

    let device = alto.open(None)?; // Opens the default audio device
    let context = device.new_context(None)?; // Creates a default context

    // Configure listener
    context.set_position([1.0, 4.0, 5.0])?;
    context.set_velocity([2.5, 0.0, 0.0])?;
    context.set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))?;

    let source = context.new_static_source()?;
    Ok(())
}
