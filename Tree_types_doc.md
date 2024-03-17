# Overview of Red Black and AVL trees

Both of these data structures are types of Binary trees, but they are superior to regular binary search trees in a couple ways.

Serach time of a regular binary tree is O(height), which may become O(n) without balancing.

The height of a red-black and avl tree is always O(log n), because of their self balancing nature.

AVL trees are 'more balanced' than red-black trees, but cause more rotations during insertion and deletion. If your application involves frequent insertion and deletion, than red-black trees should be prefered. 
 
## Red Black Tree

Binary search tree that rebalances itself after every insertion.
Each node has two children.
Stores a node that is red upon insert, and a pointer back to its parent for rebalancing.

### Rules

    - The root node is always black
    - each other node is either red or black
    - all leaves (NULL / None) are considered black
    - a red node can only have black children 
    - any path from the root to its leaves has the same number of black nodes
    - there cannot be two adjacent red nodes

## AVL Trees

Also a self-balancing binary tree.
Each sub-tree of every node differs in height by at most one.
Every sub-tree is also an AVL tree.
Rebalancing occurs when the height of any two child sub trees of a node are greater than 1.
Rebalancing is more complex than with Red-Black trees.


## References

https://www.geeksforgeeks.org/introduction-to-red-black-tree/

https://www.programiz.com/dsa/red-black-tree

https://www.codesdope.com/course/data-structures-red-black-trees-insertion/

