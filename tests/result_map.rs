use service_locator::ServiceLocator;

trait DummyService {
    fn dummy(&self) -> usize {
        0
    }
}

impl<T> DummyService for T {}

struct DummySubsystem;

static DUMMY_SERVICE_LOCATOR: ServiceLocator<dyn DummyService + Send + Sync> = ServiceLocator::new();

#[test]
fn result_map() {
    DUMMY_SERVICE_LOCATOR.provide(Box::new(DummySubsystem));

    let value = DUMMY_SERVICE_LOCATOR
        .service()
        .map(|service| service.dummy())
        .unwrap();

    assert_eq!(value, 0);
}
