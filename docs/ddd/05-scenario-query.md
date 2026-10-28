# DDD-05: Scenario Query

## Language and ownership
This context owns **CapacitySlice, RegionalAggregate**. It publishes typed values through an explicit boundary and never shares mutable internal state across contexts.

## Aggregate invariant
SQL result is auditable. Invalid transitions return a typed error. Persistence is controlled by the application transaction so policy denial stays effect-free.

## Commands, events and queries
The context accepts bounded commands, emits an audit event when executed, and exposes a read model rendered by the product. Foreign values are translated at the boundary.

## Failure and evidence
Malformed values fail closed; dependency faults remain distinct from policy denial. The matching focused test, ADR-0005, prototype map and exact-SHA semantic review establish execution.
