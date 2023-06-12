use std::sync::RwLock;

use crate::guard::{ServiceReadGuard, ServiceWriteGuard};
use crate::error::ServiceLocatorError;

#[derive(Debug)]
pub struct ServiceLocator<T: ?Sized> {
    service: RwLock<Option<Box<T>>>,
}

impl<T: ?Sized> ServiceLocator<T> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            service: RwLock::new(None),
        }
    }

    /// Provides a service and if the service is already provided, it is replaced.
    pub fn provide(&self, service: Box<T>) {
        let mut service = Some(service);
        let mut guard = self.service.write().unwrap_or_else(std::sync::PoisonError::into_inner);
        std::mem::swap(&mut *guard, &mut service);
        #[cfg(feature = "log")]
        log::info!("Provided <{}> service", std::any::type_name::<T>());
    }

    /// Provides a service if it is not already provided.
    /// 
    /// # Errors
    /// 
    /// Returns an error if the service is already provided.
    pub fn try_provide(&self, service: Box<T>) -> Result<(), ServiceLocatorError> {
        let mut service = Some(service);
        let mut guard = self.service.write()
            .unwrap_or_else(|poison_err| {
                let mut guard = poison_err.into_inner();
                std::mem::swap(&mut *guard, &mut None);
                guard
            });
        if guard.is_none() {
            std::mem::swap(&mut *guard, &mut service);
            #[cfg(feature = "log")]
            log::info!("Provided <{}> service", std::any::type_name::<T>());
            return Ok(());
        }
        #[cfg(feature = "log")]
        log::error!("The service <{}> is already provided", std::any::type_name::<T>());
        Err(ServiceLocatorError::AlreadyProvided)
    }

    /// Returns a read guard to the service.
    /// 
    /// # Errors
    /// 
    /// Returns an error if the service is not provided yet or if a caller panicked while the service was mutably borrowed.
    pub fn service(&self) -> Result<ServiceReadGuard<T>, ServiceLocatorError> {
        let guard = self.service.read().map_err(|_| ServiceLocatorError::Poisoned)?;
        if guard.is_some() {
            return Ok(ServiceReadGuard::new(guard));
        }
        #[cfg(feature = "log")]
        log::error!("The service <{}> is not provided yet", std::any::type_name::<T>());
        Err(ServiceLocatorError::NotProvided)
    }

    /// Returns a write guard to the service.
    /// 
    /// # Errors
    /// 
    /// Returns an error if the service is not provided yet or if a caller panicked while the service was mutably borrowed.
    pub fn service_mut(&self) -> Result<ServiceWriteGuard<T>, ServiceLocatorError> {
        let guard = self.service.write().map_err(|_| ServiceLocatorError::Poisoned)?;
        if guard.is_some() {
            return Ok(ServiceWriteGuard::new(guard));
        }
        #[cfg(feature = "log")]
        log::error!("The service <{}> is not provided yet", std::any::type_name::<T>());
        Err(ServiceLocatorError::NotProvided)
    }
}
