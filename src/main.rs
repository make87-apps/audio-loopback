use make87_messages::audio::FramePcmS16le;

fn main() {
    make87::initialize();

    let topic_key = make87::resolve_topic_name("AUDIO_OUT").unwrap_or_default();
    let publisher = match make87::get_publisher::<FramePcmS16le>(topic_key) {
        Some(p) => p,
        None => {
            eprintln!("Failed to get publisher for AUDIO_IN");
            return;
        }
    };

    if let Some(topic_key) = make87::resolve_topic_name("AUDIO_IN") {
        if let Some(sub) = make87::get_subscriber::<FramePcmS16le>(topic_key) {
            sub.subscribe(move |message| {
                let _ = publisher.publish(&message);
                println!("Published audio package.");
            })
                .unwrap();
        }
    }

    make87::keep_running();
}
