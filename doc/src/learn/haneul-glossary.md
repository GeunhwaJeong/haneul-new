---
title: Haneul Glossary
---

Find terms used in Haneul defined below. Where possible, we link to a canonical definition and focus upon Haneul’s use of the term.


### Accumulator

An *accumulator* makes sure the transaction is received by a quorum of authorities, collects a quorum of votes, submits the certificate to the authorities, and replies to the client. The accumulator enables transactions to be certified. Haneul offers a Gateway service that can assume the role of accumulator and collect votes on transactions from authorities in Haneul, saving end-users bandwidth.


### Authority

An authority in Haneul plays a passive role analogous to the more active role of validators and minors in other blockchains. In Haneul,
authorities do not continuously participate in the consensus protocol but are called into action only when receiving a transaction or
certificate.

For more information, see [Authorities vs validators/miners](how-haneul-works.md#authorities-vs-validators-miners).


### Causal order

[Causal order](https://www.scattered-thoughts.net/writing/causal-ordering/) is a representation of the relationship between transactions
and the objects they produce, laid out as dependencies. Authorities cannot execute a transaction dependent on objects created by a prior
transaction that has not finished. Rather than total order, Haneul uses causal order (a partial order).

For more information, see [Causal order vs total order](how-haneul-works.md#causal-order-vs-total-order). 


### Certificate

A certificate is the mechanism proving a transaction has been approved, or certified. Authorities vote on transactions, and an aggregator collects
a Byzantine-resistant-majority of these votes into a certificate and broadcasts it to all Haneul authorities, thereby ensuring finality.


### Equivocation

Equivocation in blockchains is the malicious action of dishonest actors giving conflicting information for the same message, such as inconsistent or duplicate voting.


### Epoch

Operation of the Haneul network is temporally partitioned into non-overlapping, fixed-duration *epochs*. During a particular epoch, the set of authorities participating in the network is fixed.

For more information, see [Epochs](../build/authorities.md#epochs).


### Eventual consistency

[Eventual consistency](https://en.wikipedia.org/wiki/Eventual_consistency) is the consensus model employed by Haneul; if one honest authority
certifies the transaction, all of the other honest authorities will too eventually.


### Causal history

Causal history is the relationship between an object in Haneul and its direct predecessors and successors. This history is essential to the causal
order Haneul uses to process transactions. In contrast, other blockchains read the entire state of their world for each transaction,
introducing latency.


### Finality

[Finality](https://medium.com/mechanism-labs/finality-in-blockchain-consensus-d1f83c120a9a) is the assurance a transaction will not be revoked. This
stage is considered closure for an exchange or other blockchain transaction.


### Gas

As with other blockchains, [gas](https://www.investopedia.com/terms/g/gas-ethereum.asp) in Haneul is the currency for the cost of conducting a transaction.


### Genesis

Genesis is the initial act of creating accounts and gas objects. Haneul provides a `genesis` command that allows users to create and inspect the genesis object setting up the network for operation.

For more information, see [Genesis](../build/wallet.md#genesis).


### Gateway service

Haneul provides a Gateway service that enables third parties, say app/game developers, to route transactions on behalf of users. Because Haneul never requires
exchange of private keys, users can offload the bandwidth use of transaction submission (e.g. when operating from a mobile device) to an untrusted server.


### Multi-writer objects

Multi-writer objects are those owned by more than one account. Transactions affecting multi-writer objects require consensus in Haneul. This contrasts with
those affecting only single-writer objects, which require only a confirmation of the owner’s account contents.


### Proof-of-stake

[Proof-of-stake](https://en.wikipedia.org/wiki/Proof_of_stake) is a blockchain consensus mechanism where the voting weights of authorities or validators is proportional to a bonded amount of the network's native currency (called their stake in the network). This mitigates [Sybil attacks](https://en.wikipedia.org/wiki/Sybil_attack) by forcing bad actors to gain a large stake in the blockchain first. 


### Smart contract

A [smart contract](https://en.wikipedia.org/wiki/Smart_contract) is an agreement based upon the protocol for conducting transactions in a blockchain. In Haneul, smart contracts are written in the [Move](https://github.com/GeunhwaJeong/awesome-move) programming language.


### Single-writer objects

Single-writer objects are owned by one account. In Haneul, transactions affecting only single-writer objects owned by the same account may proceed with only a check of the sender’s account, greatly speeding transaction times.


### Total order

[Total order](https://en.wikipedia.org/wiki/Total_order) refers to the ordered presentation of the history of all transactions processed by a traditional blockchain up to a given time. This is maintained by many blockchain systems, as the only way to process transactions. In contrast, Haneul uses a causal (partial) order wherever possible and safe.

For more information, see [Causal order vs total order](how-haneul-works.md#causal-order-vs-total-order). 


### Transfer

A transfer is switching the owner address of a token to a new one via command in Haneul. This is accomplished via the
[Haneul Wallet](../build/wallet.md) command line interface. It is one of the more common of many commands
available in the wallet.

For more information, see [Transferring objects](../build/wallet.md#transferring-objects).

