// Very minimal skeleton for the kernel

#include <stdio.h>

// Values copied from cnn.rs
#define INPUT_DIM 100
#define FILTER_DIM 5 // should be factor of INPUT_DIM
#define CONV_OUT_DIM  (INPUT_DIM / FILTER_DIM)
#define CONV_LAYER_SIZE 10
#define OUT_NEURON_DIM  (CONV_OUT_DIM * CONV_OUT_DIM * CONV_LAYER_SIZE)
#define OUT_LAYER_SIZE 10

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

// out assumed to be initialized to 0
extern "C" __global__ void OutputLayer(
    const double input[OUT_NEURON_DIM], 
    const double output_layer[OUT_LAYER_SIZE][OUT_NEURON_DIM], 
    double out[OUT_LAYER_SIZE]) 
{
    int out_index = blockIdx.x;
    int layer_index = threadIdx.x;
    double prod = 0.0;
    #pragma unroll
    for (int j= 0; j<125; j++){
        int i = layer_index*125 + j; 
        prod += input[i]*output_layer[out_index][i];
    }
    atomicAdd(&out[out_index], prod);
}

// out assumed to be initialized to 0
extern "C" __global__ void ConvolutionLayerAndReLU(
    const double input[INPUT_DIM][INPUT_DIM], 
    const double conv_layer[CONV_LAYER_SIZE][FILTER_DIM][FILTER_DIM], 
    double out[CONV_LAYER_SIZE][CONV_OUT_DIM][CONV_OUT_DIM]) 
{
    int filter_index = blockIdx.x;
    int filter_row_index = threadIdx.x;
    int filter_col_index = threadIdx.y;

    double prod = 0.0;
    #pragma unroll
    for (int y = 0; y < FILTER_DIM; y++)
        #pragma unroll
        for (int x = 0; x < FILTER_DIM; x++)
            prod += input[filter_row_index*FILTER_DIM + y][filter_col_index*FILTER_DIM + x] * conv_layer[filter_index][y][x];

    // ReLU
    prod = fmax(prod, 0.0);
    out[filter_index][filter_row_index][filter_col_index] = prod;
}