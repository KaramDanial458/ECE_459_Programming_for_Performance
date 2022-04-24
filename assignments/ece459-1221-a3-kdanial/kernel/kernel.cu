// Very minimal skeleton for the kernel

#include <stdio.h>

#define INPUT_DIM 100
#define FILTER_DIM 5 // should be factor of INPUT_DIM
#define CONV_OUT_DIM INPUT_DIM/FILTER_DIM
#define CONV_LAYER_SIZE 10
#define OUT_NEURON_DIM CONV_OUT_DIM * CONV_OUT_DIM * CONV_LAYER_SIZE
#define OUT_LAYER_SIZE 10


// Use atomic add function provided in lecture 22
__device__ double atomicAdd(double * address, double val) {
	unsigned long long int * address_as_ull = (unsigned long long int *) address;
	unsigned long long int old = * address_as_ull, assumed;

	do {
		assumed = old;
		old = atomicCAS(address_as_ull, assumed, __double_as_longlong(val + __longlong_as_double(assumed)));
	} while (assumed != old);

	return __longlong_as_double(old);
}

extern "C" __global__ void Convolution(double input[INPUT_DIM][INPUT_DIM], double conv_filter[CONV_LAYER_SIZE][FILTER_DIM][FILTER_DIM], double conv_output[CONV_LAYER_SIZE][CONV_OUT_DIM][CONV_OUT_DIM]) {
	double result = 0.0;
	/*
	int row = threadIdx.x / CONV_OUT_DIM;
	int col = threadIdx.x % CONV_OUT_DIM;
	*/
	int row = threadIdx.x;
	int col = threadIdx.y;
	int current_filter = blockIdx.x;

	for (int j = 0; j < FILTER_DIM; j++) {
		for (int i = 0; i < FILTER_DIM; i++) {
			result += conv_filter[current_filter][j][i] * input[row * FILTER_DIM + j][col * FILTER_DIM + i]; 
		}
	}

	conv_output[current_filter][row][col] = result;
}

extern "C" __global__ void LinearRectifier(double conv_output[CONV_LAYER_SIZE][CONV_OUT_DIM][CONV_OUT_DIM]) {
	/*
	int row = threadIdx.x / CONV_OUT_DIM;
	int col = threadIdx.x % CONV_OUT_DIM;
	*/
	int row = threadIdx.x;
    int col = threadIdx.y;
	int current_filter = blockIdx.x;

	if (conv_output[current_filter][row][col] < 0.0) {
		conv_output[current_filter][row][col] = 0.0;
	}
}

extern "C" __global__ void Output(double input[OUT_NEURON_DIM], double weights[OUT_LAYER_SIZE][OUT_NEURON_DIM], double output_vec[OUT_LAYER_SIZE]) {
	double result = 0.0;
	int current_layer = threadIdx.x;
	int current_output_position = blockIdx.x;
	
	for (int i = 0; i < 40; i++) {
		int tmp = current_layer * 40 + i;
		result += input[tmp] * weights[current_output_position][tmp];
	}

	atomicAdd(&output_vec[current_output_position], result);
}

