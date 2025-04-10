#ifndef __RELO_HELPERS_H__
#define __RELO_HELPERS_H__

#if defined(__bpf__)
#pragma clang attribute push(__attribute__((preserve_access_index)), apply_to = record)
#endif

typedef long long unsigned int u64;

#define inline __attribute__((always_inline))

struct task_struct {
	u64 start_time;
};

inline u64 *task_struct_start_time(struct task_struct *task) {
	return &task->start_time;
}

#if defined(__bpf__)
#pragma clang attribute pop
#endif

#endif
