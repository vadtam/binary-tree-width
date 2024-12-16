# Binary Tree Width
## Problem statement
First, we would like you to implement an algorithm to determine the width of a binary tree. The width of a tree is defined as the number of edges on the longest path between any two nodes. Ideally we can talk about runtimes and potential pitfalls in our call. Use a programming language of your choice and where you feel comfortable in explaining the details of your work.
## Remarkable properties of the binary tree
- Assume N is the number of nodes in the given tree. Then the possible tree width is always within interval [2*log2_N-1:N-1] for N > 0.
- Graph orientation invariance. The result does not depend on the tree orientation (ie what node to consider as root) 
## First solution: Recursion
This solution works with O(N) complexity, but is prone to stackoverflow. What a misery!
## Second solution: Iteration
The iterative depth first search with right to left traversal has the same complexity O(N) but without side issues of having a stackoverflow hazard. This is better for production.
