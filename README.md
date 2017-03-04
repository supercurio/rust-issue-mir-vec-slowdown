# Test and demo for rust-lang issue #40267
https://github.com/rust-lang/rust/issues/40267

This uses an intentionally simplified implementation of a 2nd order Biquad filter,
which is at the center of most audio DSP software.
The code is modeled after a working example but stripped of what's not strictl
required in order to make the MIR and ASM outputs easier to read.

The demo however replicates the same performance loss observed with the real
Biquad IIR filter.
The issue here probably affects negatively quite a few of projects out here
today, provided as they're using `resize()` or `extend_from_slice()` anywhere in the code.

It is possible that the regression was not noticed when testing on advanced CPUs,
most x86-64 falling in that category.
Similarily, the reduction of performance on armv7/aarch64 is negligible on the Nexus 9
Denver CPU.
But on more simple CPUs, the performance hit is larger:
Measured at 14% loss on Raspberry Pi 3.

### About build-and-run-tests.sh

This script will build and run the crate using cargo and conditional compilation of
the instructions which slow down everything.