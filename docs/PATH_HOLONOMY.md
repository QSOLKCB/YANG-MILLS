# Phase 1D — paths, parallel transport, and holonomy

## Scope

Phase 1D connects the repository's already-frozen lattice path semantics to a classical continuum path/connection dictionary. The goal is not to add a continuum solver. The goal is to make the mathematical objects and ordering conventions explicit enough that later continuum/formal work cannot silently use a different transport convention from the lattice code.

This phase establishes:

- oriented paths and their tangents;
- local pullback of a connection along a path;
- the project parallel-transport ordering convention;
- reversal, concatenation, and orientation-preserving reparameterization rules;
- open endpoint covariance versus closed-loop gauge invariance;
- `SU(3)` arbitrary lattice-path holonomy and Wilson loops;
- periodic Polyakov-loop gauge invariance;
- the small-loop bridge to curvature;
- a firewall between gauge curvature and ordinary curve geometry.

It does **not** numerically solve continuum parallel transport, extract a static potential, establish an area law, or claim confinement.

## Continuum path object

Let

```text
gamma : [0,1] -> M
```

be an oriented piecewise-smooth path on a smooth manifold `M`. Wherever `gamma` is differentiable, its tangent is

```text
gamma_dot(t) in T_{gamma(t)} M.
```

After choosing a local trivialization of the gauge bundle, let

```text
A in Omega^1(U, g)
```

be the local Lie-algebra-valued connection one-form. Its pullback along the path is the one-dimensional Lie-algebra-valued coefficient

```text
a_gamma(t) = A_{gamma(t)}(gamma_dot(t)).
```

The local object `A` depends on the chosen trivialization; the resulting parallel transport has the corresponding endpoint gauge-covariance law.

## Frozen transport convention

The Phase 1A lattice authority `YM-WILSON-LOOP-V1` already fixes an ordering rule: path segments are multiplied **left to right in traversal order**. Phase 1D therefore chooses the continuum convention that matches it.

The transporter satisfies

```text
dU_gamma(t)/dt = U_gamma(t) a_gamma(t),
U_gamma(0) = I.
```

Equivalently, write the solution schematically as

```text
U_gamma = P_R exp(integral_0^1 a_gamma(t) dt),
```

where `P_R` records that later infinitesimal factors accumulate on the right, matching the frozen discrete product order.

Under the local gauge-transformation convention

```text
A^Omega = Omega A Omega^-1 - dOmega Omega^-1,
```

the transporter transforms as

```text
U_gamma^Omega
  = Omega(gamma(0))
    U_gamma
    Omega(gamma(1))^-1.
```

For the unitary matrix representations used here, `Omega^-1 = Omega^dagger`.

Other texts may use the opposite multiplication order, an opposite sign in the exponential, or a connection acting on the other side. Those are convention-equivalent only after all associated formulas are changed consistently. Phase 1D does not mix them.

## Composition, reversal, and reparameterization

If `gamma2 o gamma1` means that `gamma1` is traversed first and `gamma2` second, then the frozen start-left convention gives

```text
U_{gamma2 o gamma1} = U_gamma1 U_gamma2.
```

This order is the direct continuum analogue of multiplying lattice links from left to right as they are traversed.

Reversing the oriented path gives

```text
U_{gamma^-1} = U_gamma^-1.
```

For `SU(N)` matrices, the inverse is the Hermitian conjugate.

If `phi:[0,1]->[0,1]` is an orientation-preserving smooth reparameterization with `phi(0)=0` and `phi(1)=1`, then

```text
U_{gamma o phi} = U_gamma.
```

The reason is the ordinary chain rule: the pullback coefficient acquires the compensating `phi_dot` factor while the oriented image path and ordering are unchanged. Orientation reversal is not an orientation-preserving reparameterization; it invokes the inverse rule above.

## Open and closed transport

An open Wilson line / path transporter is **not** gauge invariant. It transforms at both endpoints:

```text
U_gamma^Omega
  = Omega(start) U_gamma Omega(end)^-1.
```

When `gamma` is closed, the endpoint factors coincide and the holonomy transforms by conjugation. Therefore the normalized trace is gauge invariant:

```text
W_N(C) = (1/N) Tr U_C.
```

For the frozen Phase 1A `SU(2)` observable, the repository uses

```text
W_2(C) = (1/2) Re Tr U_C.
```

For the Phase 1D `SU(3)` closed-path surface, the repository preserves the full normalized complex trace

```text
W_3(C) = (1/3) Tr U_C.
```

on an individual configuration. Its real part may be used by later observables where explicitly specified.

## Lattice discretization

The discrete oriented-step rules remain exactly those frozen by `YM-WILSON-LOOP-V1`:

```text
+mu : use U_mu(x), then x -> x+mu
-mu : first x -> x-mu, then use U_mu(x-mu)^dagger.
```

The Phase 1D `SU(3)` implementation adds:

- arbitrary open path transport with final periodic coordinate;
- arbitrary closed-path holonomy;
- normalized complex Wilson-loop trace;
- path reversal;
- fallible bounded path concatenation;
- rectangular paths for later static-potential work;
- endpoint covariance tests under local gauge transformations.

The Phase 1D path step limit is `2^20`, matching the frozen Phase 1A bounded reference surface.

## Polyakov holonomy

A Polyakov loop is the normalized trace of a holonomy that wraps one complete periodic lattice extent in a chosen direction.

Under an ordinary **periodic local gauge transformation**, the holonomy is conjugated at its basepoint, so the normalized trace is gauge invariant.

This is different from the genuine center transformation introduced in Phase 1C. That transformation is not treated as an ordinary periodic local gauge transformation and rotates the Polyakov loop by a center phase. The two transformation laws must remain distinct.

## Small-loop bridge to curvature

For the frozen local connection convention,

```text
F_A = dA + A wedge A.
```

For an infinitesimal oriented loop bounding a small surface element spanned by tangent directions `X,Y`, the holonomy has the schematic expansion

```text
U_boundary
  = I + F_A(X,Y) * oriented_area
    + higher-order terms.
```

This is the continuum reason plaquette holonomy contains information about field strength at small lattice spacing.

The repository therefore forbids the following shortcuts:

```text
finite loop == pointwise curvature        [false]
finite plaquette == continuum F_A         [false]
large numerical Wilson loop == theorem    [false]
```

Any later continuum-limit statement must specify the limiting regime and control its errors.

## Curve-geometry firewall

The word "curvature" appears in several unrelated constructions. Phase 1D keeps them separate:

```text
Frenet curvature of a path
    !=
path torsion
    !=
osculating-circle / osculating-plane geometry
    !=
gauge curvature F_A.
```

A special theorem could connect such objects in a specifically constructed model, but no such connection is assumed here.

## Static-potential future surface

Phase 1D only prepares deterministic rectangular path geometry. A later statistical phase may study a regime schematically written

```text
<W(R,T)> ~ exp(-V(R) T)
```

for sufficiently large Euclidean `T`, with a specified ensemble, uncertainty treatment, and finite-volume/discretization analysis.

Phase 1D does **not** evaluate the ensemble expectation, extract `V(R)`, establish an area law, or infer confinement.

## Claim boundary

Phase 1D provides classical path/transport definitions and bounded finite-lattice implementation/numerical evidence. It does not construct continuum quantum Yang-Mills theory and does not prove a positive mass gap.
