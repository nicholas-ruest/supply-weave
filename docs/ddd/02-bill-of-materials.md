# DDD-02: Bill of Materials

## Language and ownership
This context owns **BomEdge, Assembly, Tier**. It publishes typed values through an explicit boundary and never shares mutable internal state across contexts.

## Aggregate invariant
Explosion is acyclic and overflow-safe. Invalid transitions return a typed error. Persistence is controlled by the application transaction so policy denial stays effect-free.

## Commands, events and queries
The context accepts bounded commands, emits an audit event when executed, and exposes a read model rendered by the product. Foreign values are translated at the boundary.

## Failure and evidence
Malformed values fail closed; dependency faults remain distinct from policy denial. The matching focused test, ADR-0002, prototype map and exact-SHA semantic review establish execution.
