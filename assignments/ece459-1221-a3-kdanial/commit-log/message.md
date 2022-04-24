Adding Concurrent CUDA-based Implementation of CNN

This pull request is intended to extend the implementation of a CNN to the GPU and executing the 
learning process concurrently. The I/O functionality writing to CSV files was originally 
provided and remains unedited. The rustacuda library is used to set up the environment necessary
to communicate to the computer's GPU.

Changes pertaining to this PR are contained in two files: "cuda.rs" and "kernel.cu". Evidently,
cuda.rs contains the rust code necessary to set up the cuda context, access the GPU, and act as 
a mediator receiving calls from other portions of rust code and sending the heavy duty processing
to be done by the GPU (kernel.cu). The cuda file consists of a CudaContext struct housing all the
information relevant to the GPU's operation, an init function to populate the struct while identifying
the GPU, and a compute function responsible for calling convolution, linear rectifying, and addition.

To complement the instructions provided by cuda.rs, kernel.cu contains four functions to help feed images
through the CNN and obtain an output. The four functions are atomicAdd, Convolution, LinearRectifier,
and Output. Since threads are executed concurrently, operations with the GPU must be atomic to avoid
race conditions. Therefore, we use the atomicAdd function provided in the course notes. The convolution
function performs a dot product between the respective rows and columns of matrices at each layer of the
CNN filter. The LinearRectifier ensures all values are greater than 0 by setting negative computations to 
0. Finally, the output function implements an output layer consisting of 10 neurons containing a weight
vector. Each output neuron flattens the matrces' output from the previous layer and concatenates them.
These layers were programmed to match specifications outlined in the lab manual.

Apart from course notes, resources (whether documentation, lecture notes,  or source code) were used 
to realize and implement concepts regarding BlockSize, OutputVec, launch!, cuda programming, and cuda 
context establishment in rust (e.g. rustaCuda).

These results were tested using accompanying python scripts, "compare.py" and "generate.py". In the event
of a successful implementation compare.py prints "Comparison finished". Otherwise, the script prints two
different numbers indicating a deviation from expected values between cpu and cuda implementations. To test
different values, generate.py is used to create CSVs with new input values. After running generate.py and 
computing CNNs for the CPU and GPU-based implementation, compare.py is called once again to make sure the
comparison is still satisfied - which is was.  

To test the code for performance, the time to execute the CNN was noted for both the CPU and GPU-based 
processes. As expected, the GPU typically took approximately half the time to complete the task when 
compared to the CPU-based implementation. 
