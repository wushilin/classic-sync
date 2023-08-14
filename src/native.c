#include "native.h"

int c_init_semaphore(const SemPtr * target, int count) {
    if(target->sem_ptr != NULL) {
        return SEM_INIT_DOUBLE_INIT;
    }
    sem_t* mutex = (sem_t*) malloc(sizeof(sem_t));
    int result = sem_init(mutex, 0, count);
    if(result == 0) {
        SemPtr* target1 = target;
        target1->sem_ptr = mutex;
        return result;
    } else {
        free(mutex);
        return result;
    }
}

int c_p_semaphore(const SemPtr* target) {
    int result = 0;
    if(target->sem_ptr == NULL) {
        return SEM_P_V_ON_UNINIT;
    }
    result = sem_wait(target->sem_ptr);
    return result;
}

int c_v_semaphore(const SemPtr* target) {
    int result = 0;
    if(target->sem_ptr == NULL) {
        return SEM_P_V_ON_UNINIT;
    }
    result - sem_post(target->sem_ptr);
    return result;
}

int c_destroy_semaphore(const SemPtr* target) {
    int result = 0;
    SemPtr* target1 = target;

    result = sem_destroy(target->sem_ptr);
    free(target->sem_ptr);
    target1->sem_ptr = NULL;
    return result;
}

const int64_t UNIT = 1000000000;

int c_p_semaphore_timed(const SemPtr* target, int64_t nanoseconds) {
    int err_number = 0;
    int result;
    if(target->sem_ptr == NULL) {
        return SEM_P_V_ON_UNINIT;
    }
    struct timespec timeout;
    clock_gettime(0, &timeout);
    timeout.tv_nsec += nanoseconds;

    if(timeout.tv_nsec >= UNIT) {
        int64_t secs = timeout.tv_nsec / UNIT;
        int64_t nano = timeout.tv_nsec % UNIT;
        timeout.tv_nsec = nano;
        timeout.tv_sec += secs;
    }
    //set_normalized_timespec(&timeout);

    result = sem_timedwait((sem_t*)target->sem_ptr, &timeout);
    if( result == 0) {
        return result;
    }

    if(errno == ETIMEDOUT) {
        return SEM_E_TIMEOUT;
    }
    return errno; 
}



int c_ptbarrier_init(const PTBarrier* target, int count) {
    int result = 0;
    pthread_barrier_t* barrier = malloc(sizeof(pthread_barrier_t));
    result = pthread_barrier_init(barrier, NULL, count);
    PTBarrier* target1 = target;
    target1->ptr = barrier;
    return result;
}
int c_ptbarrier_wait(const PTBarrier* target) {
    return pthread_barrier_wait(target->ptr);
}
int c_ptbarrier_destroy(const PTBarrier* target) {
    return pthread_barrier_destroy(target->ptr);
}