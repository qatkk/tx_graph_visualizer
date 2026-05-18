# Transaction Graph Visualizer for BDK Wallets

This is a simple tool to visualize the transaction graph of a bdk wallet. The repository currently contains a simple example containing most of the scenarios the visualizer covers.

- Canonical chain is represented with different colors
- Confirmed/unconfirmed transactions are represented with different colors
- External outputs are represented in the graph

<p align="center">
  <img src="examples/simple_graph.png" width="600"><br>
  An example of a simple visualization of the transaction graph
</p>

---

### Add to Your Project

Add this to your `Cargo.toml`:

```toml
[dependencies]
tx_graph_visualizer = { git = "https://github.com/qatkk/tx_graph_visualizer" }
```

---

## Quick Start

### Run the Example

Currently the only supported visualization is .dot for graphviz package. To export the images of the graphs please install graphviz.
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
```

<!-- ### Use in Your Code


---

## Use Cases -->

---

## Rendering Formats

Once you have a `.dot` file, render it in multiple formats:

```bash
# PNG (recommended for most use cases)
dot -Tpng graph.dot -o graph.png

# SVG (scalable, good for web)
dot -Tsvg graph.dot -o graph.svg

# PDF (professional reports)
dot -Tpdf graph.dot -o graph.pdf

# ASCII (terminal viewing)
dot -Tascii graph.dot
```

---
