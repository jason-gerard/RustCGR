# RustCGR
RustCGR is a contact graph routing (CGR) library implemented in rust. This isn't a production ready library and is used for the purposes of learning and relative comparison of different contact graph routing algorithms.

## Credits
All the work in this repository is using Juan Fraire's project pyCGR https://bitbucket.org/juanfraire/pycgr/src/master/ as reference and inspiration.

You can check out his paper / tutorial on contact graph routing https://www.sciencedirect.com/science/article/abs/pii/S1084804520303489 which outlines how contact graph routing works and its use cases.

## Algorithms
[] Dijkstra's: compute the single best route

[] Depth first search: compute all routes using DFS

[] Yen's algorithm: compute the best k routes

[] Anchor search: compute route list using Anchor search

[] First ended: time based search

[] First depleted: capacity oriented search

## Program Input Contract

The input to a contact graph routing algorithm will be the contact plan which is a list of individual contacts with the format below (CSV).

Any line starting with a `#` will be ignored.
```
[+|-]<start>,[+|-]<end><from>,<to>,<rate>,<range>
```