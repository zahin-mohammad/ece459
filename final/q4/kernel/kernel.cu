#include <stdio.h>


#if __CUDA_ARCH__ < 600
__device__ double atomicAdd(double* address, double val)
{
    unsigned long long int* address_as_ull =
                              (unsigned long long int*)address;
    unsigned long long int old = *address_as_ull, assumed;

    do {
        assumed = old;
        old = atomicCAS(address_as_ull, assumed,
                        __double_as_longlong(val +
                               __longlong_as_double(assumed)));

    // Note: uses integer comparison to avoid hang in case of NaN (since NaN != NaN)
    } while (assumed != old);

    return __longlong_as_double(old);
}
#endif

// Use this as the random seed in the pseudorandom function (see instructions)
#define SEED 20210418
#define NUM_VOTERS 100000
#define NUM_SIMULATIONS 500000
#define VOTE_BLOCK_INCREMENT 3125 // 100k / 32
// Pseudo-random number generator. This provides consistent reproducible results so that
// you can test your program.
__device__ float pseudorandom( uint x ) {
    uint value = x;
    value = (value ^ 61) ^ (value>>16);
    value *= 9;
    value ^= value << 4;
    value *= 0x27d4eb2d;
    value ^= value >> 15;
    return (float) value / (float) INT_MAX;
}

extern "C" __global__ void Vote(
    const float input[NUM_VOTERS][3], 
    uint out[NUM_SIMULATIONS][2]) 
{
    // Threads Per Block: 32
    // Num Blocks : Dx = 800, Dy = 625
    int simulation_number = blockIdx.x + blockIdx.y*800;
    // printf("%d\n",simulation_number );
    int vote_block = threadIdx.x;

    int vote_a = 0;
    int vote_b = 0;

    for (int j = 0; j < VOTE_BLOCK_INCREMENT; j ++) {
        int i = vote_block*VOTE_BLOCK_INCREMENT +j;
        double voter_x = input[i][0];
        double voter_y = input[i][1];

        double p = pseudorandom(SEED+simulation_number+i);
        // Should remove these if's
        if (p < voter_x) {
            vote_a += 1;
        } else if (voter_x <= p && p < voter_x + voter_y) {
            vote_b += 1;
        }
    }
    atomicAdd(&out[simulation_number][0], vote_a);
    atomicAdd(&out[simulation_number][1], vote_b);
}
