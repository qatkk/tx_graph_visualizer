# Transaction Graph Visualizer for BDK Wallets

This is a simple tool to visualize the transaction graph of a bdk wallet. Currently the only supported visualization is .dot for graphviz package. To export the images of the graphs please install graphviz.
For mac:
```bash
     brew install graphviz
```
For Ubuntu/Debian:
```bash
    sudo apt install graphviz
```
To run the example and see the corresponding graph run the following commands:
```bash
    brew install 
    cargo build 
    cargo --example simple
    dot -Tpng graph.dot -o graph.png
```