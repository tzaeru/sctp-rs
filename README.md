# sctp-rs
A Rust interface for SCTP. Prefers a native implementation, alternative implementations under consideration.

## Summary

SCTP (https://en.wikipedia.org/wiki/Stream_Control_Transmission_Protocol) is a message-based networking protocol with optional reliability and in-sequence ordering.

Initially, this crate aims to provide an interface to native implementations of SCTP. Due to the moderate adoption rates of SCTP, it is not always available on all platforms, such as most smartphones. Therefore an alternative implementation of SCTP over UDP tunneling is considered.

Currently heavily WIP and may not compile nicely!

Windows only for now, you need SctpDrv (http://www.bluestop.org/SctpDrv/) to compile.

Idea is to provide a simple interface to SCTP functions in lib.rs.

Native implementations go to "native" and can be transparently included in lib.rs depending on platform.

Super heavy WIP!