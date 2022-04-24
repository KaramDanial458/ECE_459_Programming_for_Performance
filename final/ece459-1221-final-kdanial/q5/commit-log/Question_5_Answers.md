#Question 5

#Identifying Targets

After running "make", we observe the generated `flamegraph` to have 4 functions exhibiting frequent visits by the program: drop, insert, search, and print. 
These functions exhibit approximately a large percentage of samples - 5.62%, 28.10%, 46.79%, and 9,86% respectively to be thorough. The speed up one might 
have in decreasing the frequency of each function is a 74.89% decrease in samples. We cannot say for sure the amount of execution time saved from the 
`flamegraph` but we do know significantly less visits can be made to these functions to improve performance by decreasing the overall samples.

#Challenges

The largest part of simsearch is the SimSearch<Id>::search function, which only calls the SimSearch<Id>::search_tokens funciton (one-to-one dependency). 
The search_tokens function uses two hashmaps to manage the SimSearch's token and a series of sequential calculations to return a vector of result ids. 
This sequential software implementation makes it very challenging to parallelize its functionality. For example, if we were to implement a multi-threaded
version of this function, we would need to do the gruesome job of ensuring each thread correctly writes to the two hashmaps. This function is also tightly 
coupled with SimSearch's properties, which increases the risk of making this function multithreaded. In summary, all operations are sequentially reliant and
intertwined making this task (functional code segment) very challenging to parallelize.

#Output Time
Please refer to the CHANGES file in the q5 directory.

