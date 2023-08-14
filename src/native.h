#ifndef COOL_H
#define COOL_H
#include <stdio.h>
#include <pthread.h>
#include <semaphore.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>
#define _POSIX_C_SOURCE 200112L
//#include <linux/time.h>

#include <errno.h>
#include <signal.h>

#define SEM_P_V_ON_UNINIT -90021
#define SEM_INIT_DOUBLE_INIT -90022
#define SEM_E_TIMEOUT -90022

typedef struct {
    intptr_t sem_ptr;
} SemPtr ;

typedef struct {
    intptr_t ptr;
} PTBarrier;

int c_ptbarrier_init(const PTBarrier* target, int count);
int c_ptbarrier_wait(const PTBarrier* target);
int c_ptbarrier_destroy(const PTBarrier* target);
int c_init_semaphore(const SemPtr* target, int count);

int c_p_semaphore(const SemPtr* target);

int c_v_semaphore(const SemPtr* target);

int c_p_semaphore_timed(const SemPtr* target, int64_t nanos);

int c_destroy_semaphore(const SemPtr* target);
#endif