use std::ops::{Deref, DerefMut};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

/// RAII structure used to release the shared read access to the service when dropped.
/// 
/// This structure is created by the [`service`][super::ServiceLocator::service] method on [`ServiceLocator<T>`][super::ServiceLocator].
#[derive(Debug)]
pub struct ServiceReadGuard<'a, T: ?Sized + 'a> {
    service: RwLockReadGuard<'a, Option<Box<T>>>,
}

impl<'a, T: ?Sized + 'a> ServiceReadGuard<'a, T> {
    #[inline]
    pub(crate) fn new(service: RwLockReadGuard<'a, Option<Box<T>>>) -> ServiceReadGuard<'a, T> {
        Self {
            service,
        }
    }
}

impl<'a, T: ?Sized> Deref for ServiceReadGuard<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: The service is checked not to be None in the ServiceLocator<T>::service() method.
        self.service.as_ref().unwrap()
    }
}

/// RAII structure used to release the exclusive write access to the service when dropped.
/// 
/// This structure is created by the [`service_mut`][super::ServiceLocator::service_mut] method on [`ServiceLocator<T>`][super::ServiceLocator].
#[derive(Debug)]
pub struct ServiceWriteGuard<'a, T: ?Sized + 'a> {
    service: RwLockWriteGuard<'a, Option<Box<T>>>,
}

impl<'a, T: ?Sized> ServiceWriteGuard<'a, T> {
    #[inline]
    pub(crate) fn new(service: RwLockWriteGuard<'a, Option<Box<T>>>) -> Self {
        Self {
            service,
        }
    }
}

impl<'a, T: ?Sized> Deref for ServiceWriteGuard<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: The service is checked not to be None in the ServiceLocator<T>::service_mut() method.
        self.service.as_ref().unwrap()
    }
}

impl<'a, T: ?Sized> DerefMut for ServiceWriteGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The service is checked not to be None in the ServiceLocator<T>::service_mut() method.
        self.service.as_mut().unwrap()
    }
}
