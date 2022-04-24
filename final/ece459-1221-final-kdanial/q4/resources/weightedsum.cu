#include <cmath>

#define SIZE 1022

#define twoD(x,y) ((x)*(SIZE+2)+(y))
#define fourD(x1,y1,x2,y2) ((((x)*(SIZE+2)+(y))*2+(x2))*2+(y2))

extern "C"  __global__ void add(const float *P, const float *A, float *sums) {
    int x = blockIdx.x; int y = blockIdx.y;
	x = threadIdx.x;

	for (int y = 1; y < SIZE + 2; y++) {
		sums[twoD(x,y)] = P[twoD(x,y)] + A[fourD(x,y,1,0)]*P[twoD(x,y-1)]
                                       + A[fourD(x,y,0,1)]*P[twoD(x-1,y)]
                                       + A[fourD(x,y,1,2)]*P[twoD(x,y+1)]
                                       + A[fourD(x,y,2,1)]*P[twoD(x+1,y)];

		printf("P VALUE %f\n", P[twoD(x,y)]);
		printf("Calculated value Sum: %f\n", sums[twoD(x,y)]);
	}
}

// you may find it helpful to have out as a parameter for debugging purposes, but you don't need it in the final version
// you are also free to drop N if you want.
extern "C" __global__ void find_max_index(float *out, int *out_idx, float *sums) {
    *out = 0;

    for (int x = 0; x < SIZE + 2; x++) {
        for (int y = 0; y < SIZE + 2; y++) {
            if (sums[twoD(x,y)] > *out) {
				*out = sums[twoD(x,y)];
                *out_idx = twoD(x,y);
				printf("NEW MAX: %f\n", *out);
            }
        }
    }
}
