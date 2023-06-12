use criterion::{black_box, criterion_group, criterion_main, Criterion};
use service_locator::ServiceLocator;
use std::ops::{Deref, DerefMut};

trait AudioSubsystem {
    fn play(&mut self);
    fn stop(&mut self);
    fn is_playing(&self) -> bool;
}

#[derive(Debug, Default)]
struct MidiPlayer {
    playing: bool,
}

impl AudioSubsystem for MidiPlayer {
    fn play(&mut self) {
        self.playing = true;
    }

    fn stop(&mut self) {
        self.playing = false;
    }

    fn is_playing(&self) -> bool {
        self.playing
    }
}

#[derive(Debug, Default)]
struct Mp3Player {
    playing: bool,
}

impl AudioSubsystem for Mp3Player {
    fn play(&mut self) {
        self.playing = true;
    }

    fn stop(&mut self) {
        self.playing = false;
    }

    fn is_playing(&self) -> bool {
        self.playing
    }
}

static AUDIO_SUBSYSTEM: ServiceLocator<dyn AudioSubsystem + Send + Sync> = ServiceLocator::new();

pub fn bench(c: &mut Criterion) {
    let mut group_deref = c.benchmark_group("deref-and-deref_mut");
    group_deref.sample_size(10_000);
    group_deref.warm_up_time(std::time::Duration::from_secs(10));
    group_deref.measurement_time(std::time::Duration::from_secs(30));

    AUDIO_SUBSYSTEM.provide(Box::new(MidiPlayer::default()));

    group_deref.bench_function("service-deref", |b| {
        let service = AUDIO_SUBSYSTEM.service().unwrap();
        b.iter(|| {
            // Repeated to avoid ERROR: At least one measurement of [...] took zero time per iteration. This should not be possible.
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
        })
    });

    group_deref.bench_function("service_mut-deref", |b| {
        let service = AUDIO_SUBSYSTEM.service_mut().unwrap();
        b.iter(|| {
            // Repeated to avoid ERROR: At least one measurement of [...] took zero time per iteration. This should not be possible.
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
            black_box(service.deref());
        })
    });

    group_deref.bench_function("service_mut-deref_mut", |b| {
        let mut service = AUDIO_SUBSYSTEM.service_mut().unwrap();
        b.iter(|| {
            // Repeated to avoid ERROR: At least one measurement of [...] took zero time per iteration. This should not be possible.
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
            black_box(service.deref_mut());
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
