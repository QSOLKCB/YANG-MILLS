#!/usr/bin/env python3
"""Fail-closed checks for the Phase 1E classical geometry surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

EXPECTED_CLASSICAL_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-CLASSICAL-GEOMETRY-V1",
    "status": "phase-1e-review-candidate",
    "scope": "classical continuum bundle, connection, curvature, Hodge, and Yang-Mills dictionary",
    "depends_on": [
        {"contract_id": "YM-PATH-HOLONOMY-V1", "required_status": "phase-1d-frozen"},
        {"contract_id": "YM-SU3-ALGEBRA-V1", "required_status": "phase-1b-algebra-frozen"},
        {"contract_id": "YM-SU3-LATTICE-V1", "required_status": "phase-1b-frozen"},
    ],
    "definition_ids": {
        "base_geometry": "YM-BASE-GEOMETRY-V1",
        "differential_forms": "YM-DIFFERENTIAL-FORMS-V1",
        "principal_bundle": "YM-PRINCIPAL-BUNDLE-V1",
        "associated_bundle": "YM-ASSOCIATED-BUNDLE-V1",
        "bundle_section": "YM-BUNDLE-SECTION-V1",
        "gauge_connection": "YM-GAUGE-CONNECTION-V1",
        "covariant_derivative": "YM-COVARIANT-DERIVATIVE-V1",
        "gauge_curvature": "YM-GAUGE-CURVATURE-V1",
        "hodge_star": "YM-HODGE-STAR-V1",
        "parallel_transport_dictionary": "YM-PARALLEL-TRANSPORT-DICTIONARY-V1",
        "yang_mills_functional": "YM-YANG-MILLS-FUNCTIONAL-V1",
        "gauge_transformation": "YM-GAUGE-TRANSFORMATION-V1",
        "lattice_continuum_dictionary": "YM-LATTICE-CONTINUUM-DICTIONARY-V1",
    },
    "base_geometry": {
        "base": "smooth manifold M",
        "tangent_bundle": "TM=disjoint_union_x T_xM",
        "cotangent_bundle": "T^*M=disjoint_union_x T_x^*M",
        "four_dimensional_yang_mills_background": "oriented Riemannian 4-manifold when Hodge/self-duality formulas are invoked",
        "base_metric_is_gauge_fiber_metric": False,
    },
    "differential_forms": {
        "k_form_type": "Omega^k(M,W) when W is the relevant coefficient bundle",
        "scalar_k_form": "alternating k-linear covariant tensor field",
        "two_form": "alternating bilinear form on pairs of tangent vectors",
        "wedge_product_is_graded": True,
    },
    "principal_bundle": {
        "type": "pi:P->M",
        "group": "compact Lie group G",
        "action": "free right principal action R_g:P->P",
        "fiber_is_not_tangent_space": True,
        "connection_form": "omega_P in Omega^1(P,g)",
        "connection_vertical_reproduction": "omega_P(xi_sharp)=xi",
        "connection_equivariance": "R_g^* omega_P=Ad_{g^-1} omega_P",
    },
    "associated_bundle": {
        "type": "E=P x_rho V",
        "representation": "rho:G->GL(V)",
        "equivalence": "(p,v)~(p g,rho(g^-1)v)",
        "section": "s:M->E with pi_E o s=id_M",
        "matter_or_representation_field": "section of an associated bundle",
        "principal_connection_is_section": False,
    },
    "local_gauge_convention": {
        "local_section": "sigma:U->P",
        "local_potential": "A=sigma^* omega_P in Omega^1(U,g)",
        "section_change": "sigma'=sigma g",
        "standard_section_change_rule": "A'=g^-1 A g+g^-1 d g",
        "phase1d_variable": "Omega=g^-1",
        "phase1d_rule": "A^Omega=Omega A Omega^-1-dOmega Omega^-1",
        "associated_local_field_rule": "psi^Omega=rho(Omega) psi",
        "local_potential_is_global_ordinary_one_form_on_nontrivial_bundle": False,
    },
    "covariant_derivative": {
        "global_type": "nabla_A:Gamma(E)->Gamma(T^*M tensor E)",
        "local_matrix_rule": "D_A=d+rho_*(A)",
        "leibniz": "nabla_A(f s)=d f tensor s+f nabla_A s",
        "gauge_covariance": "D_{A^Omega}(rho(Omega)psi)=rho(Omega)D_A psi",
    },
    "curvature": {
        "global_type": "F_A in Omega^2(M,ad P)",
        "local_matrix_rule": "F_A=dA+A wedge A",
        "local_bracket_rule": "F_A=dA+(1/2)[A wedge A]",
        "gauge_covariance": "F_{A^Omega}=Omega F_A Omega^-1",
        "bianchi": "D_A F_A=0",
        "gauge_curvature_is_riemann_curvature": False,
        "gauge_curvature_is_path_frenet_curvature": False,
        "gauge_curvature_is_surface_gaussian_curvature": False,
    },
    "su3_lie_algebra_bridge": {
        "mathematical_su3": "traceless anti-Hermitian 3x3 matrices",
        "phase1b_hermitian_generators": "T^a=lambda_a/2",
        "anti_hermitian_basis": "i T^a",
        "local_connection_expansion": "A=i A_mu^a T^a dx^mu for real coefficients in the fundamental representation",
    },
    "riemannian_hodge": {
        "hodge_source": "Riemannian metric plus orientation on M",
        "hodge_type": "*:Omega^k(M,W)->Omega^(4-k)(M,W) in the four-dimensional Euclidean test convention",
        "two_form_square": "*^2=+1 on 2-forms in oriented Riemannian 4D",
        "four_dimensional_two_form_adjoint": "D_A^* F=-* D_A *F",
        "fiber_inner_product": "Ad-invariant positive-definite Lie-algebra inner product chosen independently of the base metric",
        "hermitian_fiber_metric_is_base_riemannian_metric": False,
    },
    "four_dimensional_reference_fixture": {
        "space": "oriented Euclidean R^4 with orthonormal coframe (e0,e1,e2,e3)",
        "orientation": "e0 wedge e1 wedge e2 wedge e3 positive",
        "two_form_basis_order": ["01", "02", "03", "12", "13", "23"],
        "hodge_basis_rule": ["*01=23", "*02=-13", "*03=12", "*12=03", "*13=-02", "*23=01"],
        "purpose": "finite-dimensional sign/convention regression only; not a continuum solver",
    },
    "flatness_and_parallel_transport": {
        "flatness": "F_A=0",
        "local_flatness_rule": "on sufficiently small simply connected trivializing neighborhoods a flat connection is locally pure gauge",
        "flat_implies_globally_trivial_holonomy": False,
        "phase1d_start_left_transport": "U_gamma=P_gamma^-1",
        "forward_transport": "P_gamma maps associated-fiber coordinates from gamma(0) to gamma(1)",
        "forward_transport_gauge_rule": "P_gamma^Omega=rho(Omega(end)) P_gamma rho(Omega(start))^-1",
        "phase1d_start_left_gauge_rule": "U_gamma^Omega=rho(Omega(start)) U_gamma rho(Omega(end))^-1",
        "multi_chart_rule": "subdivide path into trivializing charts and insert transition-function coordinate changes at chart boundaries",
    },
    "levi_civita_firewall": {
        "connection": "nabla^LC on TM is the unique torsion-free metric-compatible connection determined by the base Riemannian metric",
        "gauge_connection_equals_levi_civita": False,
        "riemann_curvature_equals_gauge_curvature": False,
        "gauge_parallel_transport_equals_levi_civita_parallel_transport": False,
    },
    "yang_mills_functional": {
        "background": "oriented Riemannian 4-manifold with Ad-invariant Lie-algebra inner product",
        "normalization": "YM(A)=(1/2) integral_M |F_A|^2 dvol",
        "euler_lagrange": "D_A^* F_A=0",
        "yang_mills_connection": "critical point of YM(A)",
        "every_critical_point_is_global_minimizer": False,
    },
    "self_duality": {
        "self_dual": "F_A=+*F_A",
        "anti_self_dual": "F_A=-*F_A",
        "decomposition": "F_A=F_A^+ + F_A^- with F_A^+=(F_A+*F_A)/2 and F_A^-=(F_A-*F_A)/2",
        "bianchi_implication": "in 4D, F_A=+/-*F_A together with D_A F_A=0 implies D_A^*F_A=0",
        "quantum_mass_gap_implication": False,
    },
    "lattice_continuum_dictionary": {
        "link": "U_mu(x) approximates the Phase1D start-left transporter U_gamma along one oriented lattice edge; forward transport is its inverse",
        "plaquette": "oriented plaquette product is a finite small-loop holonomy whose smooth small-spacing expansion contains F_mu_nu",
        "wilson_loop": "normalized trace of discrete closed holonomy approximates the corresponding continuum holonomy observable under an appropriate continuum limit",
        "wilson_action": "finite-spacing Wilson action is a lattice discretization; equality with the continuum Yang-Mills functional at nonzero lattice spacing is not claimed",
        "finite_plaquette_equals_pointwise_curvature": False,
        "finite_lattice_result_implies_continuum_quantum_theorem": False,
    },
    "topological_branch": {
        "instantons_and_chern_classes_in_scope_as_current_mass_gap_evidence": False,
        "donaldson_theory_in_scope_as_current_mass_gap_evidence": False,
        "may_be_added_later_with_separate_contract": True,
    },
    "continuum_quantum_theory_constructed": False,
    "positive_mass_gap_proved": False,
    "cross_domain_semantic_ontology_imported": False,
    "claim_scope": "classical differential-geometric definitions and finite-dimensional convention regressions only",
}

EXPECTED_CLASSICAL_VECTORS: dict[str, Any] = {
    "contract_id": "YM-CLASSICAL-GEOMETRY-V1",
    "vector_set": "CLASSICAL-GEOMETRY-PHASE1E-V1",
    "status": "phase-1e-review-candidate",
    "vectors": [
        {
            "id": "hodge-star-basis-signs",
            "fixture": "oriented Euclidean R^4 orthonormal coframe",
            "basis_order": ["01", "02", "03", "12", "13", "23"],
            "expected_map": ["23", "-13", "12", "03", "-02", "01"],
            "claim_class": "C0",
        },
        {"id": "hodge-star-square-two-form", "input": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0], "observable": "max_abs(**F-F)", "expected": 0.0, "tolerance": 0.0, "claim_class": "C0"},
        {"id": "hodge-inner-product-wedge-identity", "alpha": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0], "beta": [6.0, -5.0, 4.0, -3.0, 2.0, -1.0], "observable": "coefficient(alpha wedge *beta)-inner_product(alpha,beta)", "expected": 0.0, "tolerance": 0.0, "claim_class": "C0"},
        {
            "id": "self-dual-decomposition",
            "input": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            "self_dual_expected": [3.5, -1.5, 3.5, 3.5, 1.5, 3.5],
            "anti_self_dual_expected": [-2.5, 3.5, -0.5, 0.5, 3.5, 2.5],
            "star_self_dual_residual_max": 0.0,
            "star_anti_self_dual_residual_max": 0.0,
            "reconstruction_residual_max": 0.0,
            "claim_class": "C0",
        },
        {"id": "self-anti-orthogonality", "input": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0], "observable": "inner_product(F_plus,F_minus)", "expected": 0.0, "tolerance": 0.0, "claim_class": "C0"},
        {"id": "phase1d-forward-transport-dictionary", "source_contract": "YM-PATH-HOLONOMY-V1", "phase1d_start_left": "U_gamma", "forward_associated_transport": "P_gamma", "relation": "U_gamma=P_gamma^-1", "expected": "exact-semantic-match", "claim_class": "C0"},
        {"id": "su3-hermitian-to-antihermitian-lie-algebra-bridge", "source_contract": "YM-SU3-ALGEBRA-V1", "hermitian_generator": "T^a=lambda_a/2", "su3_lie_algebra_basis": "i T^a", "expected": "traceless-anti-Hermitian-basis", "claim_class": "C0"},
    ],
}


def main() -> None:
    validate_exact(
        "contracts/YM-CLASSICAL-GEOMETRY-V1.json",
        load("contracts/YM-CLASSICAL-GEOMETRY-V1.json"),
        EXPECTED_CLASSICAL_CONTRACT,
    )
    validate_exact(
        "test-vectors/classical-geometry-v1.json",
        load("test-vectors/classical-geometry-v1.json"),
        EXPECTED_CLASSICAL_VECTORS,
    )
    print("Phase 1E classical geometry contract checks: passed")


if __name__ == "__main__":
    main()
