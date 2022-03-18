---
title: Haneul Glossary
---

Find terms used in Haneul defined below. Where possible, we link to a canonical definition and focus upon Haneul’s use of the term.


### Accumulator

An [accumulator](https://en.wikipedia.org/wiki/Accumulator_(cryptography)) makes sure the transaction is received by a quorum of authorities,
collects a quorum of votes, submits the certificate to the authorities, and replies to the client. The accumulator enables transactions to be
certified. Haneul offers a Gateway service that can assume the role of accumulator and collect votes on transactions from authorities in Haneul,
saving end users bandwidth.


### Authority

An authority in Haneul plays a passive role analogous to the more active role of validators and minors in other blockchains. In Haneul,
authorities do not continuously participate in the consensus protocol but are called into action only when receiving a transaction or
certificate.

For more information, see [Authorities vs validators/miners](how-haneul-works.md#authorities-vs-validators-miners).


### Causal order

[Causal order](https://www.scattered-thoughts.net/writing/causal-ordering/) is a representation of the relationship between transactions
and the objects they produce, laid out as dependencies. Authorities cannot execute a transaction dependent on objects created by a prior
transaction that has not finished. Rather than total order, Haneul uses causal order.

For more information, see [Causal order vs total order](how-haneul-works.md#causal-order-vs-total-order). 


### Certificate

A certificate is the mechanism proving a transaction has been approved, or certified. Authorities vote on transactions, and the sender collects
a Byzantine-resistant-majority of these votes into a certificate and broadcasts it to all Haneul authorities, thereby ensuring finality.


### Equivocation

Equivocation in blockchains is the malicious action of dishonest actors giving conflicting information, such as inconsistent or duplicate voting.


### Epoch

Operation of the Haneul network is temporally partitioned into non-overlapping, fixed-duration (e.g. 24-hour) *epochs*. During a particular epoch, the set of authorities participating in the network is fixed.

For more information, see [Epochs](../build/authorities.md#epochs).


### Eventual consistency

[Eventual consistency](https://en.wikipedia.org/wiki/Eventual_consistency) is the consensus model employed by Haneul; if one honest authority
certifies the transaction, all of the other honest authorities will too eventually.


### Family history

Family history is the relationship between an object in Haneul and its direct predecessors and successors. This history is essential to the causal
order Haneul uses to process transactions. In contrast, other blockchains attempt to read the entire state of their world for each transaction,
introducing great latency.


### Finality

[Finality](https://medium.com/mechanism-labs/finality-in-blockchain-consensus-d1f83c120a9a) is the assurance a transaction will not be revoked. This
stage is considered closure for an exchange or other blockchain transaction.


### Gas

As with other blockchains, [gas](https://www.investopedia.com/terms/g/gas-ethereum.asp) in Haneul is the currency for the cost of conducting a transaction.


### Genesis

Genesis is the initial act of creating accounts and gas objects. Haneul provides a `genesis` command that 

For more information, see [Genesis](../build/wallet.md#genesis).


### Gateway service

Haneul provides a Gateway service that enables third parties, say app/game developers, to route transactions on behalf of users. Because Haneul never requires
exchange of private keys, third parties may offload bandwidth use from mobile device to server - for a fee.


### Multi-writer objects

Multi-writer objects are those owned by more than one account. Transactions affecting multi-writer objects require consensus in Haneul. This contrasts with
those affecting only single-writer objects, which require only a confirmation of the owner’s account.


### Proof-of-stake

[Proof-of-stake](https://en.wikipedia.org/wiki/Proof_of_stake) is a blockchain consensus mechanism where the voting weights of authorities or validators is
proportional to their stake in the network. This mitigates attacks by forcing bad actors to gain a large stake in the blockchain first. 


### Smart contract

A [smart contract](https://en.wikipedia.org/wiki/Smart_contract) is an agreement based upon the protocol for conducting transactions in a blockchain. In Haneul,
smart contracts are written in the [Move](https://github.com/GeunhwaJeong/awesome-move) programming language.


### Single-writer objects

Single-writer objects are owned by one account. In Haneul, transactions affecting only single-writer objects owned by the same account may proceed with only a
check of the sender’s account, greatly speeding transaction times.


### Total order

Total order is the view of the entire state of a blockchain at any given time. This is used by many blockchain systems to certify transactions. In contrast,
Haneul uses causal order.

For more information, see [Causal order vs total order](how-haneul-works.md#causal-order-vs-total-order). 


### Transfer

A transfer is switching the owner address of a token to a new one via command in Haneul. This is accomplished via the
[Haneul Wallet](../build/wallet.md) command line interface. It is one of the more common of many commands
available in the wallet.

For more information, see [Transferring objects](../build/wallet.md#transferring-objects).
