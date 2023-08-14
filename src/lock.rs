use crate::semaphore::{Semaphore, SemaphoreError};
use std::time::Duration;

/// ReentrantLock implemented using Semaphore(1).
/// 
/// This Lock does not need to be mutable on calling, it is safe to share across multiple threads even with read only mode.
/// 
/// Example:
/// ```
/// use std::sync::Arc;
/// use std::time::Duration;
/// use std::thread;
/// use classic_sync::lock::ReentrantLock;
/// let lock = ReentrantLock::new();
/// let arc_lock = Arc::new(lock);
/// for i in 0..3 {
///     let lock_copy = Arc::clone(&arc_lock);
///     let tid = i;
///     thread::spawn(move || {
///         lock_copy.lock();
///         println!("Now we are in critical section!");
///         std::thread::sleep(Duration::from_secs(3));
///         // Other people can't acquire the lock even when I am sleeping.
///         lock_copy.unlock(); // You have to manually unlock it to release the lock
///     });
/// }
/// ```
pub struct ReentrantLock {
    sem:Semaphore
}


/// Implementation of ReentrantLock
impl ReentrantLock {
    /// Create a new Lock object. Lock can be shared using Arc<ReentrantLock> with readonly access
    pub fn new() -> ReentrantLock {
        let sem = Semaphore::new(1);
        return ReentrantLock {
            sem
        };
    }

    /// Acquire the lock and wait indefinitely for it to happen
    /// Calling on already locked lock will block forever (dead lock)
    /// It is calling internal semaphore's p() operation effectively.
    pub fn lock(&self)  {
        self.sem.p();
    }

    /// Release the lock. Do not call unlock multiple times. It will give non-exclusive access any more!
    /// It is calling internal semaphore's v() operation effectively.
    pub fn unlock(&self) {
        self.sem.v();
    }

    /// Try to acquire a lock. Return true if lock is acquired. Return false if acquire timed out.
    pub fn try_lock(&self, timeout:Duration) -> bool {
        return self.sem.p_timeout(timeout);
    }
}
