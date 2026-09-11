# References

## Problem definition

- Arthur Jaffe and Edward Witten, **Quantum Yang–Mills Theory**, official Clay Mathematics Institute problem description: https://www.claymath.org/wp-content/uploads/2022/06/yangmills.pdf
- Clay Mathematics Institute, **Yang–Mills & the Mass Gap** overview: https://www.claymath.org/millennium/yang-mills-the-maths-gap/

## Lattice starting point

- Kenneth G. Wilson, **Confinement of Quarks**, *Physical Review D* 10 (1974), 2445–2459. DOI: `10.1103/PhysRevD.10.2445`.

## Classical geometry orientation

These MathWorld entries are orientation/reference leads for terminology and dependency mapping. They are not substitutes for primary literature or standard graduate texts when theorem statements, sign conventions, or formalisation surfaces are frozen.

- Wolfram MathWorld, **Principal Bundle**: https://mathworld.wolfram.com/PrincipalBundle.html
- Wolfram MathWorld, **Fiber Bundle**: https://mathworld.wolfram.com/FiberBundle.html
- Wolfram MathWorld, **Vector Bundle Connection**: https://mathworld.wolfram.com/VectorBundleConnection.html
- Wolfram MathWorld, **Curvature**: https://mathworld.wolfram.com/Curvature.html
- Wolfram MathWorld, **Vector Field**: https://mathworld.wolfram.com/VectorField.html
- Wolfram MathWorld, **Manifold**: https://mathworld.wolfram.com/Manifold.html
- Wolfram MathWorld, **Second Fundamental Form**: https://mathworld.wolfram.com/SecondFundamentalForm.html
- Wolfram MathWorld, **Fundamental Forms**: https://mathworld.wolfram.com/FundamentalForms.html
- Wolfram MathWorld, **Cyclic Group C3**: https://mathworld.wolfram.com/CyclicGroupC3.html

Additional orientation material supplied for the roadmap covers 2-forms, bundle sections, vector and tangent bundles, Hermitian metrics, the Levi-Civita connection, parallel transport, zero sections, and covariant derivatives. These concepts are used only after their role is independently restated in Yang–Mills notation.

The project should preserve the dependency map

```text
smooth oriented Riemannian base M
        |
        | supplies TM, T*M, differential forms, metric g, Hodge star *
        v
principal G-bundle P -> M
        |
        | representation rho:G -> GL(V)
        v
associated vector bundle E = P x_rho V
        |
        | principal connection A induces nabla_A / d_A
        v
sections, parallel transport, holonomy
        |
        v
F_A in Omega^2(M, ad P)
```

and must separately preserve the distinction

```text
Levi-Civita connection on TM     !=     Yang-Mills gauge connection on P
Riemann curvature of (M,g)       !=     gauge curvature F_A
Christoffel symbols              !=     gauge potential components A_mu
```

The Hodge star used in four-dimensional self-duality is supplied by the metric and orientation on the base manifold; it does not identify the Levi-Civita and gauge connections.

Second fundamental forms and other submanifold-geometry constructions are useful background but are not current mass-gap dependencies. They remain deferred unless a later rigorous route requires them.

All Yang–Mills-specific formulas and conventions must be independently verified before promotion into a frozen contract.

## Explicit surname-collision quarantine

Number-theory objects named after **Mills**—including Mills' theorem, Mills' constant/primes, and Mills ratio—are unrelated by name alone to Robert Mills or Yang–Mills gauge theory. They are outside the research surface unless an independent mathematical connection is explicitly established.

Later phases should pin the exact literature used for update algorithms, classical bundle/connection conventions, differential forms and Hodge theory, instantons, glueball operator construction, benchmark spectra, scale setting, and continuum extrapolation before those methods are implemented.
