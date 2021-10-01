pub mod prelude;
use prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = toybox::Engine::new("franco21")?;

    loop {
        engine.process_events();
        if engine.should_quit() {
            break
        }

        engine.end_frame();
    }

    Ok(())
}


