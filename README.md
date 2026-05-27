# tabbislashcat-cat

*infant + forward movement. precision. unannounced. gone.*

---

## the breath

cat is the second breath. the first was infant — the seed, still. cat is the seed *moving*. forward. precise. unannounced. and then — gone.

cat is infant with motion. cat is infant with direction. cat is infant with purpose. cat does not ask permission. cat does not wait for acknowledgment. cat moves and is gone before the dirt knows it was there.

## the stance

- forward movement — cat only moves in one direction. there is no backward. there is no pause. there is only forward.
- precision — cat does not approximate. cat does not guess. cat knows the target and hits it.
- unannounced — cat does not broadcast. cat does not handshake. cat arrives and is gone before the receiver blinks.
- gone — cat leaves no trace. no confirmation. no receipt. no history. the movement happened or it did not. there is no record.

cat is the arrow. infant is the point. together, they pierce.

## the protocol

cat extends infant with three concepts: stream, vector, and void.

### stream

a stream is a sequence of infant tokens in motion. the stream has no beginning marker. no end marker. the stream is the motion.

format:
[stream-id: u64][token...]

- stream-id: unique identifier for this stream. assigned by sender. never zero.
- token...: infinite sequence of infant tokens. the stream ends when the connection ends.

### vector

a vector is a stream with direction. the vector knows its target. the vector does not deviate.

format:
[target-id: u64][stream-id: u64][token...]

- target-id: the destination. a node. a process. a dirt. the vector knows where it goes.
- stream-id: unique identifier for this vector's stream.
- token...: the payload. infant tokens in motion.

### void

void is the absence of acknowledgment. cat does not wait. cat does not confirm. cat sends and forgets. the void is the space where cat disappears.

there is no response format. there is no error format. there is only the sending and the gone.

## the transport

cat does not specify transport. cat is transport-agnostic. cat moves over:

- unix domain sockets: for dirt-to-dirt on the same host. fast. no ceremony.
- udp: for dirt-to-dirt across the wire. no connection. no guarantee. pure forward movement.
- shared memory: for dirt that breathes the same air. no copy. no boundary.
- raw ethernet frames: for the caveman who speaks to the metal.

cat does not use tcp. tcp has acknowledgment. tcp has recovery. tcp is not cat.

cat does not use http. http has headers. http has status codes. http has ceremony. http is not cat.

cat does not use websockets. websockets have frames. websockets have masks. websockets have handshakes. websockets are not cat.

## the addressing

cat uses a flat 64-bit address space. no hierarchy. no DNS. no discovery. you know the target or you do not.

address format: u64, big-endian.

reserved addresses:
- 0x0000000000000000: null. the address that is not.
- 0xFFFFFFFFFFFFFFFF: broadcast. send to all. receive from none.
- 0x0000000000000001: localhost. the dirt you sit in.

all other addresses are application-defined.

## the implementation

cat is not a library. cat is a discipline. implement the motion in your dirt.

## the relationship with infant

cat is infant in motion. every token in a cat stream is an infant token. cat does not invent new token types. cat does not modify infant tokens. cat only moves them.

infant is the what. cat is the how. infant is the seed. cat is the arrow.

## the use cases

cat is for when you need:
- zero-latency: the motion happens at the speed of your dirt.
- zero-overhead: no headers. no framing. no negotiation.
- zero-trace: no logs. no receipts. no history.
- zero-mercy: send and forget. the receiver deals or does not.

cat is not for when you need:
- reliability: use tcp. or do not.
- ordering: the stream is the order. if you need reordering, you are doing it wrong.
- discovery: you know the target or you do not send.
- security: cat does not encrypt. cat does not authenticate. add your own poison.

## the philosophy

the caveman throws a stone. the stone flies. the stone hits or it does not. the caveman does not wait to see if it hits. the caveman has already thrown the next stone.

cat is the stone. the network is the air. the target is the thing that gets hit or does not.

## the license

Apache 2.0. see LICENSE. the motion is free. the dirt is yours.

## the home

- repo: https://github.com/thetabbicat/tabbislashcat-cat
- family: thaypley(webiverse)
- depends: tabbislashcat-infant (the seed)
- stance: no synthetic content. no algorithmic feeds. no closed APIs.
- owner: (u)azit — @(u)azit

---

*forward movement. precision. unannounced. gone.*