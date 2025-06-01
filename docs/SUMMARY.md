# Besedarium Documentation

Welcome to the Besedarium protocol framework documentation.

## User Guides

- [Protocol Visualization Guide](protocol-viz.md) - Complete guide to creating and using protocol visualizations
- [Protocol Examples](protocol-examples.md) - Working examples of protocol implementations
- [Implementation Overview](ImplementationOverview.md) - High-level framework architecture

## Technical Documentation

- [Type-Level Programming in Rust](Type-Level%20Programming%20in%20Rust.md) - Advanced type system usage
- [Protocol Safety Heuristics](protocol_safety_heuristics.md) - Safety patterns and best practices
- [Runtime Implementation Patterns](runtime-implementation-patterns.md) - Implementation strategies
- [Duality](duality.md) - Protocol duality concepts and implementation
- [Projections](Projections.md) - Protocol projection techniques
- [Recursion](recursion.md) - Handling recursive protocols

## Advanced Topics

- [Disjointness in TPar](Disjointness_in_TPar.md) - Type-level disjointness analysis

## Planning and Reviews

- [Planning](planning/) - Project planning documents
- [Reviews](reviews/) - Code and design reviews
- [Visualization](visualization/) - Visualization-related documentation

## Quick Start

1. **New to Besedarium?** Start with the [Implementation Overview](ImplementationOverview.md)
2. **Want to visualize protocols?** Check the [Protocol Visualization Guide](protocol-viz.md)
3. **Looking for examples?** See [Protocol Examples](protocol-examples.md)
4. **Advanced usage?** Explore [Type-Level Programming in Rust](Type-Level%20Programming%20in%20Rust.md)

## Building Documentation

### Rustdoc

```bash
cargo doc --workspace --open
```

### mdBook

```bash
mdbook serve docs/ --open
```

---

For contribution guidelines and development setup, see the main repository README.
