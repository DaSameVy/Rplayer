use gstreamer::{ClockTime, Registry};
use gstreamer_play::{Play, PlayMessage, PlayVideoRenderer};

pub fn init_gstreamer() -> Result<(), Box<dyn std::error::Error>> {
    gstreamer::init()?;

    let registry = Registry::get();

    for plugin in registry.plugins() {
        if plugin.is_loaded() {
            println!(
            "Preloaded GStreamer plugin {} detected. {}",
            plugin.plugin_name(),
            plugin.description()
        );
        } else if let Err(error) = plugin.load() {
            eprintln!(
                "Failed to load GStreamer plugin {}. {error:?}",
                plugin.plugin_name()
            );
        } else {
            println!(
                "GStreamer plugin {} loaded. {}",
                plugin.plugin_name(),
                plugin.description()
            );
        }
    }

    Ok(())
}

pub fn play_video(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let play = Play::new(None::<PlayVideoRenderer>);

    play.set_uri(Some(path));
    play.play();

    std::thread::spawn(move || {
        for msg in play.message_bus().iter_timed(ClockTime::NONE) {
            match PlayMessage::parse(&msg) {
                Ok(PlayMessage::EndOfStream) => {
                    println!("Video ended.");
                    play.stop();
                    break;
                }
                Ok(PlayMessage::Error { error, details }) => {
                    eprintln!("GStreamer error: {error} ({details:?})");
                    play.stop();
                    break;
                }
                Ok(PlayMessage::Warning { error, details }) => {
                    eprintln!("GStreamer warning: {error} ({details:?})");
                }
                Ok(_) => {},
                Err(_) => unreachable!(),
            }
        }
        play.message_bus().set_flushing(true);
    });
    
    Ok(())
}