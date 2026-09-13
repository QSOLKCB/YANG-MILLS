# Phase 3A — gauge-invariant spatial glueball operators

Phase 3A begins the glueball laboratory only after the separately versioned Phase 2C2 `SU(3)` validation gate has passed.

The authority introduced here is:

```text
YM-GLUEBALL-OPERATORS-V1
status = phase-3a-candidate
basis  = YM-GLUEBALL-SPATIAL-A1PP-V1
```

It defines an **unsmeared finite-lattice spatial Wilson-loop operator basis**. It does not yet define smearing, blocking, connected correlators, vacuum subtraction, effective masses, fit windows, continuum extrapolations, or a glueball spectrum.

## Geometry

The frozen candidate convention is

```text
Euclidean time direction = 3
spatial directions       = 0,1,2
```

Every operator is built entirely from closed spatial loops on one fixed Euclidean time slice. The reference basis requires every spatial extent to be at least 3 so the `2x1` rectangle is not admitted only through immediate periodic wrapping. It also requires `Lx = Ly = Lz`; the `A1++` label is only used on a cubic periodic spatial lattice where axis permutations belong to the spatial symmetry group.

The underlying link/path orientation is inherited unchanged from `YM-SU3-LATTICE-V1` and `YM-PATH-HOLONOMY-V1`.

## Minimal scalar basis

The candidate basis contains two unsmeared elements:

1. `spatial-plaquette-1x1`
   - planar `1x1` spatial Wilson loops;
   - orientation orbit `(0,1)`, `(0,2)`, `(1,2)`.

2. `spatial-rectangle-2x1`
   - planar `2x1` spatial Wilson loops;
   - ordered orientation orbit `(0,1)`, `(0,2)`, `(1,0)`, `(1,2)`, `(2,0)`, `(2,1)`;
   - the first direction is the length-two direction.

For each element the reference observable is

```text
O(t) = arithmetic mean over spatial translations and listed orientations
       of Re[(1/3) Tr U_C].
```

The translation/orientation average is a finite-lattice scalar construction. Taking the real part gives the even orientation/parity surface and is invariant under complex conjugation for the charge-conjugation-even surface.

## `A1++` versus continuum `0++`

The finite cubic lattice does not have exact continuum rotational symmetry. The candidate is therefore labelled

```text
finite-lattice irrep      = A1++
continuum channel intent  = 0++
```

and the repository explicitly records

```text
finite_lattice_a1pp_is_exact_continuum_spin_zero = false
```

The continuum `0++` label is the intended spectroscopy channel, not an exact finite-spacing spin theorem.

## Deterministic validation surface

On an all-identity `3^4` configuration, every closed Wilson loop is identity, so both basis elements are exactly `1` on every time slice.

For one spatial `3^3` slice:

```text
1x1 plaquette count = 27 spatial sites * 3 orientations = 81
2x1 rectangle count = 27 spatial sites * 6 orientations = 162
```

The candidate also checks the complete basis before and after the deterministic local `SU(3)` gauge transformation already used by the frozen lattice/path regression surface. The maximum basis-element difference must be at most `2e-11`.

## What Phase 3A does not claim

Phase 3A does **not** claim any of the following:

- a measured glueball mass;
- a connected Euclidean correlator;
- a fitted energy level;
- a finite-volume mass gap;
- confinement;
- an exact continuum-spin assignment at finite lattice spacing;
- construction of continuum quantum Yang–Mills theory;
- a positive continuum mass gap;
- a Clay Millennium Problem solution.

## Next slices

The next scientific steps are deliberately separate authorities:

1. versioned smearing/blocking transformations;
2. connected Euclidean correlators and vacuum subtraction;
3. uncertainty-aware effective-mass analysis for the scalar channel;
4. only after validation, additional `2++` and `0-+` channels.

Separating these surfaces prevents an implementation fact about gauge-invariant operators from being silently promoted into a spectroscopy result.
