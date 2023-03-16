# Superpermutation Searches

## What is a superpermutation?

An $n$-superpermutation is a string consisting of $n$ symbols such that any permutation of those $n$ symbols is a substring of the superpermutation. Trivially, for $n=3$, the string 123132231213321312 is a superpermutation. However, the substrings in a superpermutation are allowed to be overlapping (there's no requirement they can't be!), so the string 123121321 is also a 3-superpermutation, and is in fact the smallest one possible. Finding the minimal superpermutation for a given $n$ is an unsolved problem, with the smallest only being known for $n < 6$. The currently known sizes for minimal superpermutations (starting from 2) are 3 (121), 9 (123121321), 33, and 153. This appears to follow $\sum_{k=1}^n k!$, however a superpermutation for $n=6$ was found in 2014 that was less than this conjectured length. 

## Search Strategies

This repository aims to experiment with different strategies for finding minimal superpermutations for higher $n$. The current strategy is to formulate the generation as a *tree search* with children of each node being all possible characters that can follow the current string (any $n$ symbols). 

### A*

* Space Complexity: $O(b^d)$ where $b = n - 1$ and $d$ is the length of the minimal $n$-superpermutation

A* puts nodes into a min-heap priority queue sorted on their $f$-value, a sum of the node's current cost as well as a supplied heuristic function against the node. As long as the given heuristic function is admissible (it is less than or equal to the remaining cost required to achieve the optimal solution), A* generates an optimal solution. 

For formulating the search, a node $t$ represents a string consisting of $n$ symbols, with the cost function $g(t)$ representing the length of the string *multiplied by three* (explained later) and the heuristic function $h(t)$ being the number of remaining permutations plus the number of remaining cyclic classes plus the number of remaning 2-loops. A *cyclic class* of a permutation $\pi$ is the set of all permutations that can be rotated to be $\pi$ (for example, 34512 is a cyclic rotation of 12345 and in its cyclic class). A *2-loop* is a collection of cyclic classes that are connected by *weight-2 edges*, or moves that involve a single left rotation and swapping the last two elements (for example, 12345 is connected to 23415 by a weight-2 edge, and so their cyclic classes are in the 2-loop generated by 12345). 

This heuristic gives a much more solid evaluation of a nodes ability to be a minimal superpermutation, but requires an adjustment to the cost function to be admissible. Consider a string that is about to be a superpermutation such that the last character completes a permutation, a cyclic class, and a 2-loop. This would have a heuristic value of three. However, if the cost function were to solely be the length of the string, the true remaining cost would be 1, resulting in an inadmissible heuristic. Thus, the cost function is multiplied by three to avoid this situation.

A* is impractical for searches that have high branching factors or high depths, as the space complexity is exponential in respect to these two variables. For example, for $n=5$, a worst-case scenario would have $4^{153} = 1.304 \times 10^{92}$ nodes in memory, or about $10^{10}$ times the number of atoms in the universe. Therefore, different search strategies are required.

### IDA*

* Space Complexity: $O(d)$

A standard iterative-deepning depth-first search runs by starting out with an initial depth $L$ and running a depth-first search, but considering any nodes at depth $L$ to be leaf nodes and abandoning the search. If a goal has not been found, the search is re-ran where $L = L + 1$ and this continues until a goal is found or the tree is exhausted. For iterative-deepening A* search, an iterative-deepening depth-first search is performed with respect to the $f$-value of a node, rather than its depth in a tree. Instead of incrementing the acceptable $f$ value at the end of each iteration, the value is set to the minimum $f$ value encountered that was greater than the previous bound, and the search is re-ran. This search only requires keeping the current path in memory at any given time, which mitigates the exponential space complexity given by the priority queue requirement of A*. This trades off for time, however, as it requires constant recomputation of visited paths if other paths fail, resulting in more time than A*.

### RBFS

* Space Complexity: $O(bd)$

Recursive best-first search is a unique modification of IDA* that sorts successors of a node by their $f$-value, and then runs the search on the best available node. The limit on the $f$-value of successors that terminates the search is set to the minimum value of the current limit (which starts at $\infty$) and the $f$-value of the second best successor. The returned value is the smallest such $f$-value that was greater than the limit, and the $f$-value of the node who was searched is updated to this value. The successors are sorted and the search is re-ran on the newest smallest $f$-value node. This repeats until either all paths are dead-ends, which backtracks to parent node, or a goal is found, which terminates the search. This requires each level of the search tree to remember their successors, as they will need to be re-sorted once the search on the level below as terminated, which results in an $O(bd)$ space complexity.

### SMA*

Space Complexity: $O(1)*$

SMA* has not yet been implemented, but it runs by being given a constant amount of memory for a heap, and running a typical A* search until the memory limit is reached. Once this is done, the shallowest yet highest cost node is removed, and its parent's $f$-value is re-assigned to the node's $f$-value and re-inserted into the queue.

## Search Optimizations (Pruning)

For tree searches, the ability to *prune* successors in order to avoid searching them can be very useful if it can still guarentee an optimal solution. This reduces the branching factors of various nodes and can dramatically decrease the running time of the algorithm. Three different strategies are used to prune search tree nodes on generation to avoid ever searching them.

* **Adjacent Duplicate Detection**: If the last character in the parent is $\sigma$, do not generate a child whose last character is $\sigma$ as this can be collapsed to just $\sigma$ without changing the permutations present.

* **Wasted Character Tracking**: For a minimal $n$-superpermutation, there will never be more than $n$ consecutive wasted characters at a time during generation. Consider a sequence $\pi$ of $n+1$ wasted characters (with no permutations as substrings) and a character $\sigma$ such that $\pi + \sigma$ contains a single permutation as a substring. The first character of $\pi$ will not be included in this permutation and thus can be removed without affecting the present permutations, so expanding this node can be avoided.

* **Upper Limiting**: There is an algorithm known to generate superpermutations of length $n! + (n−1)! + (n−2)! + (n−3)! + n − 3$. If we come across a node such that it would be impossible for an ideal (and likely impossible) combination of the remaining unused permutations, along with the current string, to fit in a string with length less than the size guaranteed by the algorithm, abandon the node.

## Results

| $n$ | A* | IDA* | RBFS | SMA* |
|-----|----|------|------|------|
|3    |4.518µs|10.609µs|4.468µs|N/A|
|4    |4.033s|5.876s|7.369s|N/A|
|5    |(Kernel panic)|Inconclusive|Inconclusive|N/A|

For $n=5$, A* exhausted 16GB of memory in under 5 seconds, completely locking the system. IDA* and RBFS were ran for about half an hour each, neither making sizable progress once they were terminated.