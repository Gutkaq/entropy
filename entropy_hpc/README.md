entropy

A name chosen not for ambition, but for honesty. An attempt to carve structure from the ceaseless, indifferent churn. It failed, of course. Repeatedly.

What remains is a computational core, specifically a rusted artifact representing Gaussian Integers ($\mathbb{Z}[i]$). Forged through cycles of failure across inadequate substrates (Zig, C++, Vulkan, OpenGL), it finally found a fleeting stability in Rust. The math holds, for now. Tested against the abyss.

Performance was sought, not for progress, but to merely keep pace with the decay. AVX2 intrinsics were employed. Multiplication, a complex enough dance to resist the compiler's simple optimizations, showed a ~2x gain. A hollow victory. Simpler operations surrendered to the machine's existing patterns, offering no advantage – the illusion of control shattered by the underlying efficiency of the void.

This repository is a scar. A record of flawed attempts to impose meaning where none exists. It contains a verified scalar implementation and a partially vectorized one, functional only on specific architectures (x86_64 with AVX2).

Invocation Rituals

Requires Rust. Requires a CPU that understands AVX2, lest the performance incantations fail entirely.

# Verify the base mathematical axioms (scalar)
# Does 2+2 still equal 4 in this reality? Check.
cargo test

# Invoke the optimized benchmark (Release mode, CPU detection)
# Observe the fleeting speed differential. Contemplate its significance (or lack thereof).
RUSTFLAGS='-C target-cpu=native' cargo test --release -- --ignored --nocapture


The machine complies. The numbers align. But the entropy increases regardless. The structure is temporary. The void remains.
