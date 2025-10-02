//! Provider of [`One`].

use core::ops::{Deref, DerefMut};

/// Wrapper to handle value consumption.
///
/// This type usually acts as a smart pointer to saved value.
/// It can consume its value only once via [`take`](Self::take) method.
/// After calling `take`, all dereferences cause panic.
///
/// Internally, This type is super simple newtype of [`Option`].
/// However, it sometimes makes code simpler than using `Option` directly.
/// (Especially, types that implement [`Drop`] are a good example of this.)
///
/// # Examples
///
/// ```
/// # use only_one::prelude::*;
///
/// let mut message_box = None;
/// let mut worker = Worker::new(&mut message_box);
/// assert_eq!(worker.message(), "I am a new worker!");
///
/// worker.do_hard_work();
/// assert_eq!(worker.message(), "I am buzy!");
///
/// worker.do_bullshit_work();
/// assert_eq!(message_box.unwrap(), "I am retired!");
///
/// struct Worker<'a> {
///     message: One<String>,
///     message_box: &'a mut Option<String>,
/// }
///
/// impl<'a> Worker<'a> {
///     pub fn new(message_box: &'a mut Option<String>) -> Self {
///         let message = One::new("I am a new worker!".to_string());
///         Self {
///             message,
///             message_box,
///         }
///     }
///
///     pub fn message(&self) -> &str {
///         &self.message
///     }
///
///     pub fn do_hard_work(&mut self) {
///         *self.message = "I am buzy!".to_string();
///     }
///
///     pub fn do_bullshit_work(mut self) {
///         *self.message = "I am retired!".to_string()
///     }
/// }
///
/// impl Drop for Worker<'_> {
///     fn drop(&mut self) {
///         *self.message_box = Some(One::take(&mut self.message));
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct One<T>(Option<T>);

impl<T> One<T> {
    /// Creates an instance.
    pub fn new(value: T) -> Self {
        Self(Some(value))
    }

    /// Creates an empty instance.
    pub fn none() -> Self {
        Self(None)
    }

    /// Returns `true` if value exists.
    pub fn exists(this: &Self) -> bool {
        this.0.is_some()
    }

    /// Takes the value out of this wrapper.
    ///
    /// # Panics
    ///
    /// Panics if this value taken already.
    pub fn take(this: &mut Self) -> T {
        Self::expect(this.0.take())
    }

    /// Returns some value of argument.
    ///
    /// # Panics
    ///
    /// Panics if argument is none.
    #[track_caller]
    fn expect<V>(x: Option<V>) -> V {
        x.expect("Value taken already.")
    }
}

impl<T> Default for One<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Deref for One<T> {
    type Target = T;

    /// Dereferences the value.
    ///
    /// # Panics
    ///
    /// Panics if this value taken already.
    fn deref(&self) -> &Self::Target {
        Self::expect(self.0.as_ref())
    }
}

impl<T> DerefMut for One<T> {
    /// Mutably dereferences the value.
    ///
    /// # Panics
    ///
    /// Panics if this value taken already.
    fn deref_mut(&mut self) -> &mut Self::Target {
        Self::expect(self.0.as_mut())
    }
}

impl<T> From<T> for One<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
