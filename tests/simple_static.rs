use service_locator::ServiceLocator;

trait DummyService {
    fn dummy(&self) {}
}

impl<T> DummyService for T {}

struct DummyServiceA;

struct DummyServiceB;

static DUMMY_SERVICE_LOCATOR: ServiceLocator<dyn DummyService + Send + Sync> = ServiceLocator::new();

#[test]
fn simple() {
    // The service is not provided yet.
    assert!(DUMMY_SERVICE_LOCATOR.service().is_err());

    // Provide the service.
    DUMMY_SERVICE_LOCATOR.provide(Box::new(DummyServiceA));

    {
        // The service is now provided.
        let service = DUMMY_SERVICE_LOCATOR.service().unwrap();
        service.dummy();
    }

    DUMMY_SERVICE_LOCATOR.provide(Box::new(DummyServiceB));

    {
        // The service can be mutably borrowed.
        let service = DUMMY_SERVICE_LOCATOR.service_mut().unwrap();
        service.dummy();
    }
}
