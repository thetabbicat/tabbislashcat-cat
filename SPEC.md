# tabbislashcat-cat — technical specification

## abstract

cat is a minimal, unidirectional, fire-and-forget transport protocol built on infant tokens. cat extends infant with motion: streams, vectors, and the void. cat does not acknowledge. cat does not recover. cat sends and is gone.

## protocol overview

cat operates over various transports but defines a common wire format. the wire format consists of a header followed by infant token payloads.

## wire format

### stream frame

A stream is a unidirectional sequence of infant tokens.

wire format:
- stream-id: 8 bytes (u64, big-endian)
- tokens: zero or more infant-encoded tokens

The stream has no explicit end marker. the stream ends when the transport connection closes.

### vector frame

A vector is a stream with a specific target.

wire format:
- target-id: 8 bytes (u64, big-endian)
- stream-id: 8 bytes (u64, big-endian)
- tokens: zero or more infant-encoded tokens

The vector has no length prefix. the receiver reads until the transport indicates end-of-data.

## addressing

cat uses a flat 64-bit address space.

reserved addresses:
- 0x0000000000000000: null address (invalid)
- 0xFFFFFFFFFFFFFFFF: broadcast (send to all, receive from none)
- 0x0000000000000001: localhost (the current node)

All other addresses are application-defined.

## transport bindings

### unix domain sockets

cat can operate over unix domain sockets (SOCK_STREAM or SOCK_DGRAM).

For SOCK_STREAM: multiple vectors can be multiplexed on a single connection by using the target-id and stream-id to demultiplex.

For SOCK_DGRAM: each datagram contains exactly one vector.

### UDP

cat over UDP: each UDP packet contains exactly one vector.

wire format for UDP:
[target-id: 8 bytes][stream-id: 8 bytes][tokens...]

Maximum UDP packet size: 65507 bytes (65535 - 8 - 8 = 65519 bytes for tokens). implementations must handle packets up to this size.

### shared memory

cat can operate over shared memory (e.g., POSIX shared memory, mmap).

The shared memory region is treated as a circular buffer of vectors. each vector is prefixed with its total length (u16, big-endian) for framing.

format:
[length: 2 bytes][target-id: 8 bytes][stream-id: 8 bytes][tokens...]

### raw ethernet

cat can operate directly over ethernet frames.

ethernet type: 0x88B5 (reserved for experimental use)
payload: vector frame (target-id + stream-id + tokens)

Note: this requires raw socket access and is platform-dependent.

## transport requirements

cat does NOT use:
- TCP (has acknowledgment and flow control)
- HTTP (has headers and ceremony)
- WebSockets (has framing and handshakes)
- gRPC (has too much)

cat prefers:
- UDP (no connection, no guarantee)
- Unix domain sockets (fast, local)
- Shared memory (fastest, local)
- Raw ethernet (for the purist)

## error handling

cat has no error handling. if a vector cannot be delivered, it is dropped. if a vector is malformed, it is dropped. there is no retry. there is no notification.

## security

cat provides NO security. implementations must:
- validate all input (addresses, lengths, token streams)
- authenticate and encrypt at a higher layer if needed
- limit resource usage (memory, file descriptors)

cat is designed for trusted environments. do not expose cat to untrusted networks without additional protection.

## relationship to infant

cat is a transport layer for infant tokens. cat does not modify infant. cat does not extend infant. cat only moves infant tokens from one dirt to another.

All token encoding/decoding is performed according to the infant specification.

## use cases

cat is appropriate for:
- local IPC (inter-process communication)
- high-performance messaging
- fire-and-forget notifications
- logging and telemetry
- any scenario where zero overhead is required

cat is NOT appropriate for:
- reliable communication (use TCP or a message queue)
- request-response patterns (use HTTP, gRPC, etc.)
- untrusted networks (without additional security layers)
- scenarios requiring ordering guarantees

## implementation notes

### receiver algorithm

1. read target-id (8 bytes)
2. read stream-id (8 bytes)
3. read tokens until transport indicates end
4. for each token: decode using infant rules
5. if any decode fails: drop the entire vector, continue to next
6. deliver tokens to application

### sender algorithm

1. write target-id (8 bytes)
2. write stream-id (8 bytes)
3. for each token: encode using infant rules, write bytes
4. flush transport
5. forget the vector (no tracking, no retry)

## examples

### UDP example (sending a vector with a string token)

Target: 0x0000000000000002
Stream: 0x0000000000000001
Token: string "hello"

Wire bytes (hex):
00 00 00 00 00 00 00 02 00 00 00 00 00 00 00 01 0C 68 65 6C 6C 6F 00

Breakdown:
- target-id: 00 00 00 00 00 00 00 02 (u64 = 2)
- stream-id: 00 00 00 00 00 00 00 01 (u64 = 1)
- token: 0C (str type) + 68 65 6C 6C 6F ("hello") + 00 (null terminator)

### Unix domain socket example (multiple vectors)

Connection sends:
[vector 1][vector 2][vector 3]...

Each vector is self-contained. the receiver reads target-id and stream-id to demultiplex.

## performance considerations

cat is designed for maximum throughput and minimum latency:
- no per-message allocations (reuse buffers)
- no copying (zero-copy where possible)
- no synchronization (lock-free where possible)
- no syscalls (batch where possible)

Implementations should:
- use pre-allocated buffers
- minimize context switches
- use non-blocking I/O
- batch small vectors when possible

## the void

The void is the conceptual space where cat vectors disappear. once sent, a vector exists in the void. it may arrive. it may not. there is no knowledge. there is no tracking. the void is the essence of cat.

## versioning

cat has no version. the wire format is immutable. if changes are needed, create a new protocol.

## reference implementations

see the main README for examples. implement cat in your dirt.