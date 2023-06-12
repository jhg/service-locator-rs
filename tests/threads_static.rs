use std::sync::{Barrier, Arc};

use service_locator::ServiceLocator;

trait AudioSubsystem {
    fn play_sound(&mut self);
}

#[derive(Debug, Default)]
struct MidiPlayer {
    playing: bool,
}

impl AudioSubsystem for MidiPlayer {
    fn play_sound(&mut self) {
        self.playing = true;    }
}

#[derive(Debug, Default)]
struct Mp3Player {
    playing: bool,
}

impl AudioSubsystem for Mp3Player {
    fn play_sound(&mut self) {
        self.playing = true;
    }
}

static AUDIO_SUBSYSTEM: ServiceLocator<dyn AudioSubsystem + Send + Sync> = ServiceLocator::new();

#[test]
fn thread() {
    #[cfg(feature = "log")]
    env_logger::builder().is_test(true).filter_level(log::LevelFilter::Trace).try_init().unwrap();

    AUDIO_SUBSYSTEM.provide(Box::new(MidiPlayer::default()));

    let barrier = Arc::new(Barrier::new(3));

    let barrier_thread1 = Arc::clone(&barrier);
    let thread1 = std::thread::spawn(move || {
        AUDIO_SUBSYSTEM.service_mut().unwrap().play_sound();

        barrier_thread1.wait(); // Wait for the change of provider.

        AUDIO_SUBSYSTEM.service_mut().unwrap().play_sound();
    });

    let barrier_thread2 = Arc::clone(&barrier);
    let thread2 = std::thread::spawn(move || {
        AUDIO_SUBSYSTEM.service_mut().unwrap().play_sound();

        barrier_thread2.wait(); // Wait for the change of provider.

        AUDIO_SUBSYSTEM.service_mut().unwrap().play_sound();
    });

    AUDIO_SUBSYSTEM.service_mut().unwrap().play_sound();

    AUDIO_SUBSYSTEM.provide(Box::new(Mp3Player::default()));
    barrier.wait();
    
    AUDIO_SUBSYSTEM.service_mut().unwrap().play_sound();

    thread1.join().unwrap();
    thread2.join().unwrap();
}
