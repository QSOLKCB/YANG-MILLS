# References

## Problem definition

- Arthur Jaffe and Edward Witten, **Quantum Yang–Mills Theory**, official Clay Mathematics Institute problem description: https://www.claymath.org/wp-content/uploads/2022/06/yangmills.pdf
- Clay Mathematics Institute, **Yang–Mills & the Mass Gap** overview: https://www.claymath.org/millennium/yang-mills-the-maths-gap/

## Lattice starting point

- Kenneth G. Wilson, **Confinement of Quarks**, *Physical Review D* 10 (1974), 2445–2459. DOI: `10.1103/PhysRevD.10.2445`.

## Classical geometry orientation

These MathWorld entries are orientation/reference leads for terminology and dependency mapping. They are not substitutes for primary literature or standard graduate texts when theorem statements, sign conventions, or formalisation surfaces are frozen.

- Wolfram MathWorld, **Principal Bundle**: https://mathworld.wolfram.com/PrincipalBundle.html
- Wolfram MathWorld, **Vector Bundle Connection**: https://mathworld.wolfram.com/VectorBundleConnection.html
- Wolfram MathWorld, **Cyclic Group C3**: https://mathworld.wolfram.com/CyclicGroupC3.html

The project should preserve the distinction

```text
principal G-bundle P -> M
        |
        | representation rho:G -> GL(V)
        v
associated vector bundle E = P x_rho V
        |
        | induced connection / covariant derivative
        v
sections, parallel transport, curvature
```

and should independently verify all Yang–Mills-specific formulas and conventions before promoting them into a frozen contract.

## Explicit surname-collision quarantine

Number-theory objects named after **Mills**—including Mills' theorem, Mills' constant/primes, and Mills ratio—are unrelated by name alone to Robert Mills or Yang–Mills gauge theory. They are outside the research surface unless an independent mathematical connection is explicitly established.

Later phases should pin the exact literature used for update algorithms, classical bundle/connection conventions, instantons, glueball operator construction, benchmark spectra, scale setting, and continuum extrapolation before those methods are implemented.
