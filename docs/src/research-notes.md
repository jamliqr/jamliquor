# Research Notes

[← Back to Index](./index.md)

These are retired ideas kept as one-paragraph future-work notes, per the [2026 reassessment](../REASSESSMENT/README.md) (Gates G4 and §8). They are **not** part of JamLiquor's current scope — see [`spec.md`](../../spec.md) for the binding non-goals list. Full original documents are preserved for cleanroom provenance at [`docs/archive/AI-2025.md`](../archive/AI-2025.md) and [`docs/archive/PQC-2025.md`](../archive/PQC-2025.md).

## AI inference

The original thesis proposed embedding lightweight AI inference (TinyML-scale models) into node behavior — adaptive gas pricing, validator anomaly detection, edge-side optimization. The 2026 reassessment found no JAM-native technical justification for any of the three evaluated models (JAM-as-execution-layer, JAM-as-coordination-layer, edge-inference-marketplace): all three require extensive non-JAM infrastructure (model distribution, quantized runtimes, result attestation), which classifies AI as an application-layer concern, not a client feature. It remains a plausible idea for a future project built *on top of* a working JAM node, not inside one.

## Post-quantum cryptography (PQC)

The original thesis proposed opt-in Dilithium/Kyber/XMSS modules as drop-in replacements for JAM's classical signature schemes. JAM's protocol cryptography (Bandersnatch ring VRF, Ed25519) is fixed by the Gray Paper; a client using non-conformant signatures cannot interoperate with the rest of the network, and PQ primitives are 30–100x larger than the classical ones they'd replace — directly attacking JamLiquor's one real differentiator (smallness) with no present JAM use case. PQ migration, if it ever happens, is a spec-level decision for the Web3 Foundation, not a client-level add-on. Revisit only if the Gray Paper itself adopts PQ primitives.
