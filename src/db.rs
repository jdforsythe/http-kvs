use std::sync::{ Arc, Mutex };
use std::collections::HashMap;

use bytes::Bytes;

/// Server state shared across all connections
/// 
/// `Db` contains a `HashMap` storing the key/value data
/// 
/// A `Db` instance is a *handle* to shared state. Cloning `Db` is shallow
/// and only incurs an atomic ref count increment.
#[derive(Debug, Clone)]
pub(crate) struct Db {
  /// Handle to shared state
  shared: Arc<Shared>,
}

#[derive(Debug)]
struct Shared {
  /// The shared state is guarded by a mutex - this is a `std::sync::Mutes` and
  /// not a Tokio mutex because there are no async operations being performed
  /// while holding the mutex
  state: Mutex<State>,
}

#[derive(Debug)]
struct State {
  /// The key-value data
  records: HashMap<String, Record>,
}

/// An entry in the key-value store
#[derive(Debug)]
pub(crate) struct Record {
  /// Stored data
  data: Bytes,
}

impl Db {
  /// Create a new, empty, `Db` instance. Allocates shared state
  pub(crate) fn new() -> Db {
    let shared = Arc::new(Shared {
      state: Mutex::new(State {
        records: HashMap::new(),
      }),
    });

    Db { shared }
  }

  /// Gets the value associated with a key
  /// 
  /// Returns `None` if there is no value associated with the key.
  pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
    // Acquire the lock, get the record, and clone the value
    let state = self.shared.state.lock().unwrap();
    
    // Because data is stored using `Bytes`, a clone here is a shallow clone
    // and data is not copied
    state.records.get(key).map(|record| record.data.clone())
  }

  /// Set the value associated with a key. If a value is already associated
  /// with the key, it is overwritten
  pub(crate) fn set(&self, key: String, value: Bytes) {
    let mut state = self.shared.state.lock().unwrap();

    state.records.insert(key, Record { data: value });

    ()
  }
}
