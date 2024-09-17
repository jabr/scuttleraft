# scuttleraft

(Beginnings of) A Rust library implementing a variation of [Scuttlebutt Gossip](https://en.wikipedia.org/wiki/Gossip_protocol) and [Raft Consensus](https://en.wikipedia.org/wiki/Raft_(algorithm)).

I would like for this to be runtime agnostic, so it does not the implement the network communication layer directly. Instead, it is intended for the application to use this library to drive the logic of what to communicate, process incoming updates, and provide an API for interacting with the cluster state and metadata.

## Status

The logic for the gossip algorithm is largely complete, though the API is likely to change. I'm still experimenting with the metadata "value" abstraction in particular. There are unit tests covering most of the internal elements, but none yet on the top-level algorithm.

The initial version of the consensus layer will be coming soon, based on [my Typescript implementation](https://github.com/jabr/what-bus/blob/master/consensus.ts) and adapted to make use of the node failure detector logic in the gossip algorithm.

## References

* [Efficient Reconciliation and Flow Control
for Anti-Entropy Protocols](https://www.cs.cornell.edu/home/rvr/papers/flowgossip.pdf)
* [In Search of an Understandable Consensus Algorithm
(Extended Version)](https://raft.github.io/raft.pdf)

## License

This project is licensed under the terms of the [MIT license](LICENSE.txt).
