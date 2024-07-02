# Locality 
---


## Part C: (experimental) : Measure and explain improvements in locality


|   Angle   | row-major| col-major  |
|:---------:| :------: | :--------: |
| 180 degree| 1.50s    | 1.63s      |
| 90 degree | 1.55s    | 1.61s |
---
- I used a 51 mb 4k image for testing purposes I ran the program 10 times and calculated the averages you see above. I believe these calculations to be correct with my predictions based on that my storage is in row-major order.

## Part D (theoretical): Reason about a memory layout that might perform better

- For better cache locality and performance you would limit the amount of operations on the data structure specifically when accessing it in non-sequential order. I could alter the process of reading and processing the data in a contigous block which resembles a cache block. Other suggestions would be to consolidate my code and remove unneccessary conditionals that can cause the performance to diminish, because of the branching. I believe the only option would be to separate the two different forms of storage and the corresponding iterators for storage.

# Time:
- 6 hours on design documentation with `Marceline Kelly`

- 12 hours on implementation

- 3 hours on Readme/benchmarking

# Extra Credit Completed:

  - 270 rotation
  - transpose
  - flip horizontal
  - flip vertical