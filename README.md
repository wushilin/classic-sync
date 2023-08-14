# Super simple ReentrantLock, Semaphore and CyclicBarrier implementation
The implementation is a wrapper around C binding of Rust.

It is tested in Linux. All systems that has CC should work in theory, not tested on windows.

Most importantly the benefits are:
* They are all Sync
* They are all Unpin
* They are all Send
* They do not need to be mutable to lock!

# Semaphore

```rust
use classic_sync::semaphore::Semaphore;

let sem = Semaphore::new(10); // create semaphore with 10 concurrent access
let sem = Arc::new(sem); // Usage of Sem can be direct (rare) or by Arc (common)
let sem1 = Arc::clone(&sem); // use in other threads are done by Arc::clone

// Using semaphore
sem1.p();  // acquire token to access resource
// use resource
sem1.v(); // release resource

// Droping Arc<Semaphore> won't release all waiting threads. You have to take care of that yourself.
// After last reference is dropped, the Semaphore is destroyed. If you still have threads waiting, 
// behaivor is same as sem_t in c.
let acquire_ok:bool = sem1.p_timeout(Duration::from_secs(1)); // try to acquire with 1 seconds timeout
if acquire_ok {
    // use resource
} else {
    // acquire timeout, try again
}
```

# ReentrantLock
Nothing special, it is just a convenient wrapper for Semaphore. You could have used Semaphore::new(1) for the same purpose

```rust
use classic_sync::lock::ReentrantLock;

let lock = ReentrantLock::new(); // create exclusive lock
let lock = Arc::new(lock); // Usage of Lock can be direct (rare) or by Arc (common)
let lock1 = Arc::clone(&lock); // use in other threads are done by Arc::clone

// Using semaphore
lock1.lock();  // acquire exclusive access to resource
// use resource
lock1.unlock(); // release resource

let acquire_ok:bool = lock1.try_lock(Duration::from_secs(1)); // try to acquire with 1 seconds timeout
if acquire_ok {
    // use resource
} else {
    // acquire timeout, try again
}
```
# CyclicBarrier
If you want to sync all threads to start something together, this is the tool for you. 

It is a wrapper of pthread_barrier_t object.

```rust
use classic_sync::cyclic_barrier::CyclicBarrier;

let barrier = CyclicBarrier::new(10); // create barrier that can wait for 10 parties
let barrier = Arc::new(barrier); // Usage of Barrier can be direct (rare) or by Arc (common)
let barrier1 = Arc::clone(&barrier); // use in other threads are done by Arc::clone

// Using Barrier
barrier.wait();  // after this returns all parties are ready

// Check if this is the last one enters the party
let wr = barrier.wait();
if let Some(_) = wr {
    // This one is the last one enters the barrier
}

// after wait succeeded, barrier can be reused actually... it goes back to the pre-wait state again.

```