use crate::binding::{c_ptbarrier_init, c_ptbarrier_wait, c_ptbarrier_destroy, PTBarrier};

impl PTBarrier {
    fn init(&self, count: i32)-> i32 {
        unsafe {
            return c_ptbarrier_init(self, count);
        }
    }
    fn wait(&self) -> i32 {
        unsafe {
            return c_ptbarrier_wait(self);
        }
    }

    fn destroy(&self) -> i32 {
        unsafe {
            return c_ptbarrier_destroy(self);
        }
    }
}

/// Cyclic Barrier implementation.
/// 
/// It basically provides 2 methods:
/// 
/// let barrier = CyclicBarrier::new(count); // initialize a cyclic barrier with 3 parties
/// barrier.wait(); // Wait until all parties are wait. Returnes Ok(()) for the last one entered the state
/// 
/// Most of the time, you should use Arc(barrier).
/// 
/// You can clone the Arc container of barrier as many times as you want. 
/// // You can reuse the barrier if the config does not need to be changed.
/// 
/// 
/// drop(barrier); // discard the barrier after you are done with it. 
/// 
/// Usage:
/// Example:
/// ```
/// use std::sync::Arc;
/// use std::time::Duration;
/// use std::thread;
/// use classic_sync::cyclic_barrier::CyclicBarrier;
/// let barrier = CyclicBarrier::new(3);
/// let arc_barrier = Arc::new(barrier);
/// for i in 0..3 {
///     let barrier_copy = Arc::clone(&arc_barrier);
///     let tid = i;
///     thread::spawn(move || {
///         // if you care who's wait call triggered everyone to go, you can check the
///         // firing object returned. If it is Some(_), it triggered the barrier
///         let firing = barrier_copy.wait();
/// 
///         if firing.is_some() {
///             println!("Thread {tid} is the one triggered it!");
///         }
///         println!("Now we are starting almost at the same time!!");
///     });
/// }
/// 
/// 
/// ```
pub struct CyclicBarrier {
    bar:PTBarrier
}


/// Implementation of CyclicBarrier
impl CyclicBarrier {
    /// Create a new CyclicBarrier with count of `count`. It is backed by a C pthread_barrier_t object.
    /// count must be greater than 0. If not behavior is not determined.
    pub fn new(count:u32) ->CyclicBarrier {

        let bar = PTBarrier {
            ptr:0
        };
        let init_result = bar.init(count as i32);
        if init_result != 0 {
            panic!("pthread_barrier_t init result not 0 {init_result}");
        }
        return CyclicBarrier {
            bar
        }
    }

    /// Wait for other parties
    /// If the caller is the last one enters the barrier and triggered the barrier release
    /// Then result is Some(()). Otherwise, result is None. 
    pub fn wait(&self) -> Option<()>{
        let result = self.bar.wait();
        if result == 0 {
            return None;
        }
        if result == -1 {
            return Some(());
        }
        panic!("pthread_barrier_t wait not successful: {result}");
    }
}

/// Close internal pthread_barrier_t object on Drop
impl Drop for CyclicBarrier {
    fn drop(&mut self) {
        self.bar.destroy();
    }
}