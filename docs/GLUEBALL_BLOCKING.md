# Phase 3B2 — deterministic spatial blocking

Phase 3B2 completed the first transformation pair in the glueball laboratory by adding a separately versioned spatial coarse-link map after Phase 3B1 stout smearing. PR #13 merged cleanly, so the authority is frozen without changing its transformation semantics.

The frozen authority is:

```text
YM-GLUEBALL-SPATIAL-BLOCK-V1
status         = phase-3b2-frozen
transformation = YM-SPATIAL-STRIDE2-BLOCK-V1
```

It is intentionally simpler than a tuned spectroscopy blocking scheme. Its purpose is to preserve a transparent, gauge-covariant reference coarse-graining map that later analysis authorities may consume.

## Geometry

The fine lattice must have cubic spatial extent

```text
Lx = Ly = Lz = L
L >= 6
L even
```

with Euclidean time direction `3`. The coarse lattice has extents

```text
[L/2, L/2, L/2, Nt].
```

The `L >= 6` rule ensures that the coarse spatial extent remains at least `3`, preserving compatibility with the frozen Phase 3A `A1++` operator basis.

A coarse site `y` is embedded in the fine lattice as

```text
x = (2 y0, 2 y1, 2 y2, y3).
```

## Blocking map

For spatial directions `mu in {0,1,2}`,

```text
V_mu(y) = U_mu(x) U_mu(x + mu).
```

The order is part of the frozen semantics. No projection, averaging, or smearing is performed inside the blocking map.

The temporal link is sampled exactly at the embedded fine site:

```text
V_3(y) = U_3(x).
```

The transformation allocates the coarse lattice through the existing fallible `Su3Lattice4::new_identity` path, so allocation failure remains a `Result` error rather than an aborting full-lattice clone.

## Gauge covariance

If the fine lattice is gauge transformed by `G_f(x)`, the corresponding coarse gauge field is the restriction

```text
G_c(y) = G_f(2 y0, 2 y1, 2 y2, y3).
```

The deterministic regression checks

```text
block(U^G) = block(U)^{G_c}
```

within `5e-11` maximum link-matrix residual.

## Deterministic validation

The frozen surface requires:

- identity `6x6x6x3` blocks exactly to identity `3x3x3x3`;
- a temporal link at an embedded coarse site is copied exactly;
- all blocked links remain within the frozen `SU(3)` unitarity/determinant residual limit `5e-12`;
- the canonical fine fixture with one traceless cycle spatial link at the origin produces coarse Phase 3A operators

```text
spatial plaquette 1x1 = 0.9506172839506173
spatial rectangle 2x1 = 0.9259259259259259
```

with absolute replay tolerance `1e-12`;
- too-small, odd, or anisotropic spatial extents fail closed.

## Relationship to stout smearing

Phase 3B1 and Phase 3B2 remain separate frozen authorities. The blocking map does **not** require stout smearing. A later analysis contract may explicitly consume raw, smeared, or blocked fields, but no combined parameter tuning or overlap-improvement claim is established by Phase 3B2 itself.

## Claim boundary

Phase 3B2 itself does not claim:

- improved ground-state overlap;
- a connected Euclidean correlator;
- vacuum subtraction;
- an effective mass;
- a glueball spectrum;
- a finite-volume mass gap;
- confinement;
- continuum quantum Yang–Mills construction;
- a positive continuum mass gap;
- a Clay Millennium Problem solution.

Connected correlators are owned by the separately versioned Phase 3C authority rather than being back-propagated into this frozen transformation contract.
