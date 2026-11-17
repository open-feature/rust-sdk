use std::{
    any::Any,
    collections::HashMap,
    sync::{Mutex, TryLockError},
};

// ============================================================
//  HookData
// ============================================================

/// A mutable, string-keyed collection for passing arbitrary data between the stages of a single
/// hook during a single flag evaluation.
///
/// Each hook instance receives its own `HookData` for the duration of one evaluation, and the same
/// instance is shared across that hook's `before`, `after`, `error`, and `finally` stages. This
/// lets a hook, for example, start a timer or open a telemetry span in `before` and read it back in
/// `after`/`finally`. The data is isolated per hook: one hook can never observe another hook's data.
///
/// Values may be of any `'static + Send` type; they are stored type-erased and recovered via
/// [`HookData::get`], [`HookData::take`] or [`HookData::with_mut`], which perform a checked
/// downcast.
///
/// The SDK owns the `HookData` for the duration of an evaluation and hands hooks a shared reference
/// through [`HookContext::data`](crate::HookContext::data). The type is deliberately not [`Clone`],
/// so a hook cannot keep the data alive past the evaluation it belongs to.
///
/// # Stored types must match exactly
///
/// The type a value is stored as is inferred from the argument to [`set`](Self::set), and reads
/// must name that exact type. A string literal is stored as `&'static str`, not `String`, and an
/// unsuffixed integer literal as `i32`:
///
/// ```
/// # use open_feature::HookData;
/// let data = HookData::new();
/// data.set("id", "abc");
/// assert_eq!(data.get::<String>("id"), None);
/// assert_eq!(data.get::<&str>("id"), Some("abc"));
/// ```
///
/// See the [specification](https://github.com/open-feature/spec/blob/main/specification/sections/04-hooks.md#46-hook-data).
#[derive(Default)]
pub struct HookData {
    // INFO: `Send` alone is enough for the values: the `Mutex` is what makes `HookData` `Sync`.
    data: Mutex<Store>,
}

type Store = HashMap<String, Box<dyn Any + Send>>;

impl HookData {
    /// Create an empty [`HookData`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Lock the inner map, recovering the guard even if a previous holder panicked while holding it.
    ///
    /// Only a panic inside a [`with_mut`](Self::with_mut) closure can poison the lock; recovering
    /// rather than propagating keeps this type from ever panicking a caller.
    fn lock(&self) -> std::sync::MutexGuard<'_, Store> {
        self.data
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Set `key` to `value`, replacing any existing value for that key.
    ///
    /// The value is stored as exactly `T`, so later reads must ask for the same type (see the
    /// [type-level documentation](Self#stored-types-must-match-exactly)).
    ///
    /// Returns `&self` so calls can be chained.
    pub fn set<T: Any + Send>(&self, key: impl Into<String>, value: T) -> &Self {
        self.lock().insert(key.into(), Box::new(value));
        self
    }

    /// Get a clone of the value stored at `key`, if present and of type `T`.
    ///
    /// Returns `None` if the key is absent or the stored value is not of type `T`. For values that
    /// are not [`Clone`] (or that are expensive to clone), use [`with_mut`](Self::with_mut) or
    /// [`take`](Self::take) instead.
    pub fn get<T: Any + Clone>(&self, key: &str) -> Option<T> {
        self.lock()
            .get(key)
            .and_then(|value| value.downcast_ref::<T>())
            .cloned()
    }

    /// Remove the value stored at `key` and return it, if present and of type `T`.
    ///
    /// This gives ownership of the value without requiring `T: Clone`, which is the natural way to
    /// consume state set in an earlier stage — for example, taking an OpenTelemetry span in `after`
    /// to end it. If the key is absent or the value is not of type `T`, returns `None` and leaves
    /// the store unchanged.
    pub fn take<T: Any>(&self, key: &str) -> Option<T> {
        let mut guard = self.lock();
        // Only remove if the stored type matches, so a wrong-type call is non-destructive.
        match guard.get(key).map(|value| value.is::<T>()) {
            Some(true) => guard
                .remove(key)
                .and_then(|value| value.downcast::<T>().ok())
                .map(|boxed| *boxed),
            _ => None,
        }
    }

    /// Invoke `f` with a mutable reference to the value at `key`, if present and of type `T`, and
    /// return its result.
    ///
    /// This allows in-place mutation of non-[`Clone`] values — ending a span, pushing onto an
    /// accumulated `Vec`, etc. — without removing them from the store. Returns `None` if the key is
    /// absent or the value is not of type `T`.
    ///
    /// The internal lock is held for the duration of `f`, so `f` must not call back into the same
    /// `HookData` instance (doing so would deadlock). Formatting it with `{:?}` is the one
    /// exception: [`Debug`](std::fmt::Debug) never blocks.
    pub fn with_mut<T: Any, R>(&self, key: &str, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        self.lock()
            .get_mut(key)
            .and_then(|value| value.downcast_mut::<T>())
            .map(f)
    }

    /// Return `true` if `key` has a value set.
    pub fn contains_key(&self, key: &str) -> bool {
        self.lock().contains_key(key)
    }

    /// Remove the value stored at `key`.
    pub fn remove(&self, key: &str) {
        self.lock().remove(key);
    }

    /// Return the number of values currently stored.
    pub fn len(&self) -> usize {
        self.lock().len()
    }

    /// Return `true` if no values are stored.
    pub fn is_empty(&self) -> bool {
        self.lock().is_empty()
    }
}

impl std::fmt::Debug for HookData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // INFO: Never block here. `with_mut` holds the lock across user code, and a hook that logs
        // its context from inside that closure must not deadlock.
        let guard = match self.data.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
            Err(TryLockError::WouldBlock) => {
                return f
                    .debug_struct("HookData")
                    .field("keys", &format_args!("<locked>"))
                    .finish();
            }
        };

        // The stored values are type-erased and not necessarily `Debug`, so only expose the keys,
        // sorted so the output is stable.
        let mut keys: Vec<&String> = guard.keys().collect();
        keys.sort_unstable();
        f.debug_struct("HookData").field("keys", &keys).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_roundtrip_various_types() {
        let data = HookData::new();
        data.set("string", "value".to_string());
        data.set("int", 42_i64);
        data.set("bool", true);

        assert_eq!(data.get::<String>("string"), Some("value".to_string()));
        assert_eq!(data.get::<i64>("int"), Some(42));
        assert_eq!(data.get::<bool>("bool"), Some(true));
    }

    #[test]
    fn get_missing_key_returns_none() {
        let data = HookData::new();
        assert_eq!(data.get::<i64>("absent"), None);
    }

    #[test]
    fn get_wrong_type_returns_none() {
        let data = HookData::new();
        data.set("key", 42_i64);
        assert_eq!(data.get::<String>("key"), None);
    }

    #[test]
    fn set_overwrites_existing_value() {
        let data = HookData::new();
        data.set("key", 1_i64);
        data.set("key", 2_i64);
        assert_eq!(data.get::<i64>("key"), Some(2));
    }

    #[test]
    fn set_is_chainable() {
        let data = HookData::new();
        data.set("a", 1_i64).set("b", 2_i64);
        assert_eq!(data.get::<i64>("a"), Some(1));
        assert_eq!(data.get::<i64>("b"), Some(2));
    }

    #[test]
    fn contains_remove_len_is_empty() {
        let data = HookData::new();
        assert!(data.is_empty());
        data.set("key", 1_i64);
        assert!(data.contains_key("key"));
        assert_eq!(data.len(), 1);
        data.remove("key");
        assert!(!data.contains_key("key"));
        assert!(data.is_empty());
    }

    #[test]
    fn stored_type_must_match_exactly() {
        let data = HookData::new();
        data.set("literal", "value");
        data.set("int", 42);

        // A string literal is a `&'static str` and an unsuffixed integer an `i32`.
        assert_eq!(data.get::<String>("literal"), None);
        assert_eq!(data.get::<&str>("literal"), Some("value"));
        assert_eq!(data.get::<i64>("int"), None);
        assert_eq!(data.get::<i32>("int"), Some(42));
    }

    #[test]
    fn supports_send_only_values() {
        // `Cell` is `Send` but not `Sync`; the internal lock is what makes sharing it sound.
        let data = HookData::new();
        data.set("counter", std::cell::Cell::new(1_u64));

        data.with_mut::<std::cell::Cell<u64>, _>("counter", |counter| counter.set(2));
        assert_eq!(
            data.take::<std::cell::Cell<u64>>("counter")
                .map(|c| c.get()),
            Some(2)
        );
    }

    #[test]
    fn is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<HookData>();
    }

    #[test]
    fn debug_lists_sorted_keys() {
        let data = HookData::new();
        data.set("b", 1_i64).set("c", 2_i64).set("a", 3_i64);
        assert_eq!(format!("{data:?}"), r#"HookData { keys: ["a", "b", "c"] }"#);
    }

    #[test]
    fn debug_does_not_deadlock_inside_with_mut() {
        let data = HookData::new();
        data.set("key", 1_i64);

        let output = data.with_mut::<i64, _>("key", |_| format!("{data:?}"));
        assert_eq!(output.as_deref(), Some("HookData { keys: <locked> }"));
    }

    #[test]
    fn supports_non_primitive_values() {
        #[derive(Clone, PartialEq, Debug)]
        struct Span {
            id: u64,
        }

        let data = HookData::new();
        data.set("span", Span { id: 99 });
        assert_eq!(data.get::<Span>("span"), Some(Span { id: 99 }));
    }

    // A deliberately non-`Clone` type standing in for something like an OpenTelemetry span that
    // must be mutated in place and consumed, never cloned.
    #[derive(PartialEq, Debug)]
    struct NotClone {
        ended: bool,
    }

    #[test]
    fn take_returns_ownership_without_clone() {
        let data = HookData::new();
        data.set("span", NotClone { ended: false });

        let taken = data.take::<NotClone>("span");
        assert_eq!(taken, Some(NotClone { ended: false }));
        // The value is gone after taking.
        assert!(!data.contains_key("span"));
    }

    #[test]
    fn take_wrong_type_is_non_destructive() {
        let data = HookData::new();
        data.set("key", 42_i64);

        assert!(data.take::<String>("key").is_none());
        // Original value survives a wrong-type take.
        assert_eq!(data.get::<i64>("key"), Some(42));
    }

    #[test]
    fn take_missing_key_returns_none() {
        let data = HookData::new();
        assert!(data.take::<i64>("absent").is_none());
    }

    #[test]
    fn with_mut_mutates_non_clone_value_in_place() {
        let data = HookData::new();
        data.set("span", NotClone { ended: false });

        let result = data.with_mut::<NotClone, _>("span", |span| {
            span.ended = true;
            "ended"
        });
        assert_eq!(result, Some("ended"));

        // The mutation persisted and the value is still stored.
        assert_eq!(
            data.with_mut::<NotClone, _>("span", |span| span.ended),
            Some(true)
        );
    }

    #[test]
    fn with_mut_accumulates_across_calls() {
        let data = HookData::new();
        data.set("results", Vec::<String>::new());

        data.with_mut::<Vec<String>, _>("results", |v| v.push("first".to_string()));
        data.with_mut::<Vec<String>, _>("results", |v| v.push("second".to_string()));

        assert_eq!(
            data.get::<Vec<String>>("results"),
            Some(vec!["first".to_string(), "second".to_string()])
        );
    }

    #[test]
    fn with_mut_missing_or_wrong_type_returns_none() {
        let data = HookData::new();
        data.set("key", 1_i64);

        assert_eq!(data.with_mut::<i64, _>("absent", |v| *v), None);
        assert_eq!(data.with_mut::<String, _>("key", |v| v.clone()), None);
    }

    #[test]
    fn store_remains_usable_after_panic_in_with_mut() {
        let data = HookData::new();
        data.set("key", 1_i64);

        // `with_mut` holds the internal lock across the closure, so panicking here poisons it.
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            data.with_mut::<i64, _>("key", |_| panic!("boom"));
        }));
        assert!(panicked.is_err());

        // The store recovers the poisoned guard rather than propagating, so it stays usable.
        assert_eq!(data.get::<i64>("key"), Some(1));
        data.set("other", 2_i64);
        assert_eq!(data.get::<i64>("other"), Some(2));
    }
}
