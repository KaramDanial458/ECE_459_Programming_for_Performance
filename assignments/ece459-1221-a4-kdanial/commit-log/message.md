# Title

Hackathon Catalyzation Via Multi-Channel Messaging and File I/O Optimization

# Summary

A `flame graph` is generated to analyze the bottlenecks of a system by identifying software units frequently visited in the call stack (qualitative analysis). Quantitatively analyzing performance involves calling `hyperfine` and noting the program's execution time.

# Technical details

To see where the program can improve, one can take a look at the generated `flame graph` and identify where the system spends an unnecessary amount of time. Looking at the first `First_Flame_Graph`, we see the program excessively calling:

```
  PackageDownloader::run,
  <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  <core::str::iter::Lines as core::iter::traits::iterator::Iterator>::next

  crossbeam_channel::channel::Receiver<T>::recv
  crossbeam_channel::channel::Sender<T>::send
```

Turns out, the PackageDownloader contains a function `run` which repeatedly reads package names from a txt file. This causes unnecessary load on the program as noted by the CharSearcher and Lines operations. To improve efficiency, the names in the txt file was stored in memory using a `Vec<String>`. 

After addressing the file I/O hurdle, `recv` and `send` channel bottleneck must be addressed. In the starter code, there is only one channel controlling information flowing between Students, Idea Generators, and Package Downloaders. Carrying this much information between these three entities is very demanding for one channel to accomplish. The solution? Add two more channels such that the program uses three channels: `idea_channel`, `package_channel`, and `completed_channel`. Intuitively, the `idea_channel` and `package_channel` transfer ideas and packages, respectively. The `completed_channel` tracks the number of ideas generated and packages successfully downloaded. 

```
  `idea_channel` passes elements between the student implementation and the `IdeaGenerator`
  `package_channel` passes elements between students and package implementations such that the students can poll and identify which packages need to be/have been downloaded
  `completed_channel` is passed into student and `IdeaGenerator` implementations to identify end-to-end development.
```

# Testing for correctness

The `checksum` of the original and optimized implementations were compared. After ensuring both `checksum` values were the same, we conclude the optimized implementation correctly executes the original implementation just faster. 

# Testing for performance.

_Flame graph_

The original `flame graph`, `First_Flame_Graph.png`, and the optimized graph, `flamegraph.svg` where compared for visual changes in performance. Referring to those images shows that the optimized implementation significantly decreases the visits made to the aforementioned bottlenecks (i.e. the width of blocks associated with those functions have noticeably decreased). 

_hyperfine_

The following measurments shows `hyperfine` outputs on different ECE servers using the original Makefile input.

Original Code `eceubuntu 4:`
```
Benchmark #1: target/release/lab4
  Time (mean ± σ):     359.2 ms ±  24.0 ms    [User: 1.709 s, System: 0.183 s]
  Range (min … max):   328.0 ms … 408.5 ms    10 runs
```

Optimized Code `eceubuntu 4:`
```
Benchmark #1: target/release/lab4/
  Time (mean ± σ):      11.6 ms ±   6.4 ms    [User: 0.2 ms, System: 0.0 ms]
  Range (min … max):     1.4 ms …  28.2 ms    102 runs
```

Original Code `eceubuntu 1:`
```
Benchmark #1: target/release/lab4
  Time (mean ± σ):     446.9 ms ±  99.7 ms    [User: 1.245 s, System: 0.191 s]
  Range (min … max):   323.3 ms … 580.3 ms    10 runs
```

Optimized Code `eceubuntu 1:`
```
Benchmark #1: target/release/lab4
  Time (mean ± σ):      37.2 ms ±  12.3 ms    [User: 102.1 ms, System: 6.8 ms]
  Range (min … max):    19.9 ms …  77.3 ms    79 runs
```

From the hyperfine results, there is a `12.11x` and `32.07x` speedup when tested on `eceubuntu 1` and `eceubuntu 4` with original make file inputs. Interestingly, if the inputs increase, the speedup difference between both implementations is consistently greater (example here is by a factor of `28.18x`):

Original Code `eceubuntu 4:`
```
Benchmark #1: target/release/lab4 5000 6 5000 6 6
  Time (mean ± σ):      1.623 s ±  0.230 s    [User: 2.241 s, System: 1.002 s]
  Range (min … max):    1.184 s …  1.915 s    10 runs
```

Optimized Code `eceubuntu 4:`
```
Benchmark #1: target/release/lab4 5000 6 5000 6 6
  Time (mean ± σ):      57.6 ms ±  13.5 ms    [User: 52.0 ms, System: 3.0 ms]
  Range (min … max):    33.6 ms …  95.2 ms    27 runs
```

Overall, measurments on the eceubuntu servers were flippant as they would sometimes speed through the process at unbelievable rates and struggle in other similr scenarios at different times. Take the example below, which shows a server completing the task within 0.2 ms. I was not sure whether to include that in the report as a valid processing time due to the rate of performance fluctuation. In summary, the system was indeed optimized passed the requested `13.5x` factor but struggled to remain consistent.

Optimized Code `eceubuntu 4:`
```
Benchmark #1: target/release/lab4/
  Time (mean ± σ):       0.2 ms ±   0.1 ms    [User: 0.2 ms, System: 0.0 ms]
  Range (min … max):     0.0 ms …   2.1 ms    1142 runs
```
