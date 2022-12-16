# DS210 Final

Collaborators: None

Daniel Foley

In this project is designed to calculate what percent of the original nodes have contact with the most recent nodes via the nodes that "voted in" the Final Nodes.

This project was designed with a dataset of Wikipedia admin votes in mind with the end goal of determining the influence of the original 20 nodes on the most recently voted in nodes. The dataset was sourced from SNAP at this address: https://snap.stanford.edu/data/wiki-Vote.html
Within the dataset, there were admins voting in other admins as well as non-admin users voting.
After running my project on the dataset, I was surprised to find that none of the original 20 nodes have any contact with the final 20.
While this is a disappointing result, it speaks to the rapidity at which electoral systems overturn power in an online space.

The algorithm prints out a list of verteces of verteces. The position of the inner verteces correspond to the order in which the "final" nodes are input. The inner verteces contain f32 numbers which represent the "weight" of the "original" nodes on the final nodes annd correspond to the order in which the "original" nodes were input.

I was initially going to use breadth first search, however I quickly found it to be inefficient and cumbersome on such a large dataset where I was determining connectedness. Because of this I switched to DFS and the solution quickly became much clearer.


