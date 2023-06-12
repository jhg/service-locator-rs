# [Service locator pattern](https://gameprogrammingpatterns.com/service-locator.html) generic implementation

This is a, thread safe, generic implementation of the service locator pattern, that can be used with any trait object.

## Description

This pattern is used to provide a global access to a service, safely, without
coupling the service to the code that uses it. Read the Robert Nyström's book [Game Programming Patterns](https://gameprogrammingpatterns.com/service-locator.html) for more information about this pattern.

The key concepts of this pattern are:

- Service - An interface that defines the service (a trait).
- Service provider - An implementation of the service (implements the trait).
- Service locator - An object that locates the service provider.

## Usage

The service locator is a static object that can be used to provide and request the service. Internally, it uses a [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html) to be thread safe. The service locator is generic over the service trait object.

## Example

```rust
use service_locator::ServiceLocator;

enum Sound {
    Beep,
    Laser,
}

// The service.

trait Audio {
    fn play(&mut self, sound: Sound) {
        // Some stuff.
    }

    fn stop(&mut self, sound: Sound) {
        // Some stuff.
    }

    fn stop_all(&mut self) {
        // Some stuff.
    }

    fn is_playing(&self, sound: Sound) -> bool {
        false
    }
}

// The service providers.

#[derive(Default)]
struct SDLAudio {
    // Some stuff.
}

impl Audio for SDLAudio {
    // Use the default implementation as it is an example.
}

#[derive(Default)]
struct OpenALAudio {
    // Some stuff.
}

impl Audio for OpenALAudio {
    // Use the default implementation as it is an example.
}

static AUDIO_SERVICE_LOCATOR: ServiceLocator<dyn Audio + Send + Sync> = ServiceLocator::new();

// The service is not provided yet.
assert!(AUDIO_SERVICE_LOCATOR.service().is_err());

// Provide the service.
AUDIO_SERVICE_LOCATOR.provide(Box::new(SDLAudio::default()));

// The service is now provided.
assert_eq!(AUDIO_SERVICE_LOCATOR.service().unwrap().is_playing(Sound::Beep), false);

// The service can be mutably borrowed.
let mut service = AUDIO_SERVICE_LOCATOR.service_mut().unwrap();
service.stop_all();

// IMPORTANT: Drop the service, as it's a guard, before to request it again to avoid a deadlock.
// Before we didn't drop the service because we didn't bind the service to a variable.
drop(service);

// Change the service provider.
AUDIO_SERVICE_LOCATOR.provide(Box::new(OpenALAudio::default()));

// The service is now from the new provider.
let is_playing_laser = AUDIO_SERVICE_LOCATOR.service()
    // The closure is executed only if the service is provided,
    // and ensures that the service is dropped after the closure execution.
    .map(|service| service.is_playing(Sound::Laser))
    .unwrap();

assert_eq!(is_playing_laser, false);
```

Remember to handle the errors instead of using `unwrap()`. It's used in the example for simplicity reasons, and because it's run as a test.

## Logging

The crate uses the [`log`](https://crates.io/crates/log) crate to log. With default features to false it can be disabled.
