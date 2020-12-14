# Ray Tracer Challenge Completed project (in Rust)

My completed Ray Tracer Challenge project, written in Rust.

## Overall design

The project has been written with the intention to produce good code, so the design is overall clean and simple.

The APIs (and test suites) are similar, but not equal to the book ones; at the beginning, I thought refactoring made sense, although later I realized that for comparison purposes, matching the book structure would have made comparisons easier.

The most significant difference is that this project's renderer is parallel, therefore, significantly faster than the book reference.

Two functions are unsafe; this has been necessary in order to make the objects tree mutex-free (which allows full parallelism); this unsafe problem has been discussed on the [Rust Programming Language Forum](https://users.rust-lang.org/t/is-it-possible-to-safely-build-a-read-only-thread-safe-bidirectional-tree/52759), and there is no trivial/safe solution.

A change that would simplify the design is to convert the Shape trait into a base class; this is discussed on the trait comment. Also, it would be more readable just to have the owned version of the Matrix/Tuple operations, with a borrowed one for the exceptional cases.

The project is divided in `library`, `macros` and `practice`.

## Demo renderings (from the last chapters)

<details>
  <summary>Chapters 1-13: Shapes and properties</summary>
  <a target="_blank" rel="noopener noreferrer" href="/assets/readme/chapter13_shapes_with_effects.png?raw=true">
    <img src="/assets/readme/chapter13_shapes_with_effects.png?raw=true" width="800" style="max-width:100%;">
  </a>
</details>

<details>
  <summary>Chapter 14: Groups</summary>
  <a target="_blank" rel="noopener noreferrer" href="/assets/readme/chapter14_hexagon.png?raw=true">
    <img src="/assets/readme/chapter14_hexagon.png?raw=true" width="800" style="max-width:100%;">
  </a>
</details>

<details>
  <summary>Chapter 15: Triangles/WaveFront OBJ format</summary>
  <a target="_blank" rel="noopener noreferrer" href="/assets/readme/chapter15_astronaut.png?raw=true">
    <img src="/assets/readme/chapter15_astronaut.png?raw=true" width="800" style="max-width:100%;">
  </a>
</details>

<details>
  <summary>Chapter 16: CSG</summary>
  <a target="_blank" rel="noopener noreferrer" href="/assets/readme/chapter16_csg.png?raw=true">
    <img src="/assets/readme/chapter16_csg.png?raw=true" width="800" style="max-width:100%;">
  </a>
</details>
