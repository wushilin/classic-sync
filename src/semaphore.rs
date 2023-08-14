use std::time::Duration;

use crate::binding::{self, SemPtr, c_destroy_semaphore,c_p_semaphore_timed, c_init_semaphore, c_p_semaphore, c_v_semaphore, SEM_E_TIMEOUT, SEM_P_V_ON_UNINIT, SEM_INIT_DOUBLE_INIT};

#[derive(Debug)]
pub struct SemaphoreError {
    code:i32
}

/// Wrapper of sem_t in c. Providing Semaphore access without mut access. It is super easy to share!
/// 
/// Example:
/// ```
/// use std::sync::Arc;
/// use std::time::Duration;
/// use std::thread;
/// use classic_sync::semaphore::Semaphore;
/// let sem = Semaphore::new(3); // allows 3 concurrent access
/// let arc_sem = Arc::new(sem);
/// for i in 0..3 {
///     let sem_copy = Arc::clone(&arc_sem);
///     let tid = i;
///     thread::spawn(move || {
///         sem_copy.p();
///         println!("Now I am granted access. I should have 2 other siblings has the access at the same time!");
///         std::thread::sleep(Duration::from_secs(3));
///         // Other people can't acquire the lock even when I am sleeping.
///         sem_copy.v(); // You have to manually unlock it to release the lock
///     });
/// }
/// ```
impl SemaphoreError {
    pub fn code(&self) -> i32 {
        return self.code;
    }

    pub fn is_timeout(&self) -> bool {
        return self.code == SEM_E_TIMEOUT;
    }

    pub fn msg(&self)->String {
        if self.code == SEM_P_V_ON_UNINIT {
            "Use of uninitialized SEMAPHORE".into()
        } else if self.code == SEM_INIT_DOUBLE_INIT {
            "Can't double init SEMAPHORE".into()
        } else if self.code == SEM_E_TIMEOUT {
            "Semaphore wait timed out".into()
        } else {
            format!("SyscallError code: {}", self.code)  
        }
    }
}
impl std::fmt::Display for SemaphoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SemaphoreError code :{}, message:{}", self.code, self.msg())
    }
}

impl std::error::Error for SemaphoreError {
}

type SemaphoreResult<T> = Result<T, SemaphoreError>;

impl SemPtr {
    fn init(&self, count: i32)-> i32 {
        unsafe {
            return c_init_semaphore(self, count);
        }
    }
    fn close(&self) -> i32 {
        unsafe {
            return c_destroy_semaphore(self);
        }
    }

    fn p(&self) -> i32 {
        unsafe {
            return c_p_semaphore(self);
        }
    }

    fn p_timeout(&self, nanos:i64) -> i32 {
        unsafe {
            return c_p_semaphore_timed(self, nanos);
        }
    }
    fn v(&self) -> i32 {
        unsafe {
            return c_v_semaphore(self);
        }
    }
}

#[derive(Debug)]
pub struct Semaphore {
    ptr: SemPtr,
}

fn test<T>(what:T) where
T: Send {

}
impl Semaphore {
    pub fn new(count:i32) -> Semaphore {
        let ptr = SemPtr {
            sem_ptr: 0,
        };
        let result = ptr.init(count);
        if result != 0 {
            panic!("Semaphore init error {result}");
        }
        return Semaphore {
            ptr
        };
    }

    pub fn v(&self) {
        let result = self.ptr.v();
        if result == 0 {
            return;
        }
        panic!("v() operation on semaphore can't fail");
    }

    pub fn p_timeout(&self, duration:Duration) -> bool {
        let nanos = duration.as_nanos();
        let result = self.ptr.p_timeout(nanos as i64);
        if result == 0 {
            return true;
        }
        return false;
    }

    pub fn p(&self) {
        let result = self.ptr.p();

        if result != 0 {
            panic!("p() operation didn't work");
        }
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        self.ptr.close();
    }
}