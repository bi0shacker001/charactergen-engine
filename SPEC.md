# CharacterGen Engine Specification

Status: Draft 0.1  
Audience: contributors, integrators, client authors, and character-engine providers

## 1. Purpose

CharacterGen Engine is a self-hosted world engine for persistent, evolving characters. It provides the shared world state, time, space, perception, memory, relationships, narration, and compatibility boundaries needed for characters from different sources to coexist and develop over time.

The engine should support as many character-engine sources as possible, including:

- game engines;
- prescripted and rule-driven characters;
- local or remotely operated language models;
- hosted character services and third-party character APIs;
- human-authored or imported characters;
- future character technologies not known when this specification is written.

Its long-term goal is to enable dynamic character expansion: worlds should gain depth as characters encounter one another, accumulate shared history, develop opinions, form relationships, and introduce newly relevant people and places.

## 2. Product shape

The initial product is a single-user, self-hosted server with one or more clients. The server is authoritative. Clients may run on the same machine or connect over a private network.

This is not initially:

- a massively multiplayer world;
- a public character-chat service;
- a complete physics engine;
- an attempt to simulate every person in a fictional city;
- a system in which an LLM response automatically becomes canonical state.

The user inhabits the world but is not the only character capable of initiating action. Relevant non-player characters continue to meet, communicate, remember, and develop relationships with each other.

## 3. Normative language

The terms MUST, MUST NOT, SHOULD, SHOULD NOT, and MAY describe requirements and recommendations. A conforming implementation must satisfy all MUST and MUST NOT requirements for the implemented capability set.

## 4. Architectural invariants

1. The simulation server MUST be the authority for canonical world state.
2. Character engines and narrators MUST propose actions or interpretations; they MUST NOT directly mutate canonical storage.
3. Every committed change MUST be attributable to an event, import, administrative edit, or migration.
4. Facts, beliefs, claims, rumors, and generated suggestions MUST remain distinguishable.
5. Characters MUST only perceive information available through simulated space or an explicit communication channel.
6. Character detail MUST be bounded by relevance. Unbounded population simulation is not a goal.
7. Promotion MUST fill unknown details from available context without silently replacing established facts.
8. Provider-specific state MUST NOT become the only representation of a character.
9. A world MUST remain loadable when an optional character provider is unavailable.
10. The core data model MUST be independent of any single model vendor, game engine, or client.

## 5. System boundaries

### 5.1 World server

The world server owns:

- canonical entities and identifiers;
- simulation time and scheduled work;
- places, containment, routes, and presence;
- event validation and commitment;
- perception and communication delivery;
- character scope and lifecycle state;
- memories, beliefs, relationships, and provenance;
- narrator orchestration;
- character-engine routing;
- import processing;
- persistence, snapshots, backup, and recovery.

### 5.2 Clients

Clients MAY render maps, scenes, conversations, timelines, administrative tools, or game-specific interfaces. Clients MUST treat server responses as authoritative and MUST NOT assume that locally predicted state has been committed.

### 5.3 Character engines

A character engine portrays or advances a character through a capability contract. It may be a script, state machine, behavior tree, game-engine component, language model, remote service, or composite system.

### 5.4 Narrators

Narrators observe a deliberately scoped view of the world and propose circumstances, environmental changes, scene framing, and story beats. Narrators create opportunities and pressure; character engines determine character reactions.

## 6. World model

### 6.1 Entities

Every persistent entity MUST have:

- a stable world-local identifier;
- an entity type;
- a lifecycle state;
- creation provenance;
- a revision or version marker;
- structured attributes;
- optional source-specific extensions.

Core entity types initially include:

- world;
- character;
- location;
- route or portal;
- organization;
- household;
- object;
- communication endpoint;
- event;
- claim;
- memory;
- relationship;
- narrative thread.

### 6.2 Claims and truth

The engine MUST represent at least the following separately:

- **canonical fact:** accepted by the simulation as objectively true;
- **claim:** asserted by a source but not necessarily canonical;
- **belief:** held by a particular character with confidence and provenance;
- **rumor:** socially transmitted claim whose origin or accuracy may be uncertain;
- **proposal:** uncommitted output from a narrator or character engine.

A character's context MUST contain beliefs available to that character, not unrestricted canonical truth.

### 6.3 Provenance

Imported and generated data MUST retain:

- source identity;
- source location or reference when available;
- import or generation timestamp;
- provider and model identifiers when applicable;
- prompt, adapter, rule-set, or importer version when applicable;
- confidence;
- whether a user confirmed or pinned the result.

## 7. Space, presence, and perception

### 7.1 Spatial structure

Locations form a containment hierarchy and a connectivity graph. A character may occupy a precise location nested within broader locations, such as a room within an apartment within a building within a city.

Connections MAY encode:

- travel duration;
- capacity;
- opening hours;
- locks and access requirements;
- visibility transmission;
- acoustic transmission;
- accessibility constraints;
- transportation requirements.

### 7.2 Presence and movement

A materialized character MUST have at most one authoritative physical presence unless a source explicitly supports a different ontology. Physical movement MUST consume simulation time and follow a valid route.

### 7.3 Perception

Objective events MUST be transformed into observer-specific perceptions before entering character context or memory.

Initial perception channels:

- sound;
- sight;
- direct digital communication;
- delayed written communication.

Sound propagation SHOULD account for source volume, distance, containment boundaries, acoustic transmission, ambient noise, listener ability, and attention. The engine determines whether and how an event was perceived; a narrator MAY describe the resulting perception.

## 8. Communication

Communication occurs through explicit channels with their own delivery rules.

Initial channels:

- nearby speech;
- phone calls;
- direct text messages;
- group messages;
- email or asynchronous messages.

Delivery does not imply attention. A message may be delivered, noticed, previewed, read, ignored, misunderstood, forwarded, or answered later. These stages SHOULD be separate events where consequential.

## 9. Character representation

### 9.1 Canonical profile

A materialized character may contain:

- identity and aliases;
- immutable or pinned history;
- traits and values;
- interests, preferences, aversions, and expertise;
- needs and current condition;
- goals, commitments, and plans;
- communication style;
- relationships;
- beliefs and secrets;
- episodic and summarized memories;
- possessions, access, and communication endpoints;
- current location and schedule;
- character-engine assignment and source extensions.

The canonical profile MUST be serializable independently of the provider that currently portrays the character.

### 9.2 Relationships

Relationships MUST be directional and multidimensional. Initial dimensions SHOULD include:

- familiarity;
- affection;
- trust;
- respect;
- comfort;
- attraction;
- fear;
- resentment;
- obligation;
- interest in future contact.

Characters SHOULD also be able to hold beliefs about how another character feels toward them. Relationship changes MUST be traceable to interactions, memories, administrative edits, or imported history.

### 9.3 Memory

Memories MUST originate from perceived events, received communications, internal conclusions, imports, or explicit authoring. A memory SHOULD include participants, time, place, subjective interpretation, emotional significance, confidence, and provenance.

Summaries MAY replace detailed memories in active context, but the underlying event ledger SHOULD remain available for reconstruction and auditing.

## 10. Scoped simulation and character lifecycle

### 10.1 Social horizon

The engine maintains an active social horizon anchored initially to the player and optionally to pinned characters or active narrative threads.

Relevance is calculated from factors including:

- social-graph distance;
- direct interaction with the player;
- physical proximity;
- frequency and recency of mention;
- unresolved commitments or conflicts;
- narrative-thread participation;
- explicit pinning;
- source-specific importance.

Graph distance MUST be bounded. A distant chain of acquaintances MUST NOT cause indefinite materialization.

### 10.2 Lifecycle states

Initial lifecycle states:

- **reference:** a textual mention with unresolved identity;
- **stub:** stable identity plus known constraints;
- **supporting:** persistent profile and summarized simulation;
- **principal:** detailed goals, beliefs, memories, relationships, and active portrayal;
- **dormant:** preserved character not currently receiving active simulation;
- **archived:** retained for history but excluded from ordinary activation.

### 10.3 Promotion

When a reference or stub becomes sufficiently relevant, the engine builds a promotion context from:

- all established claims about the character;
- known relationships and statements made by related characters;
- prior appearances and perceptions;
- relevant setting and source constraints;
- imported canon;
- configured world rules;
- the target character engine's capabilities.

The selected promotion provider proposes a completed profile. A validator MUST reject contradictions with pinned facts and MUST identify unresolved conflicts rather than arbitrarily choosing one. Successful promotion is committed as an event.

### 10.4 Demotion and dormancy

Characters may become dormant when their relevance falls. Dormancy MUST preserve identity, significant history, relationships, commitments, and future scheduled events. Reactivation MUST use that preserved state.

## 11. Autonomous social evolution

Characters may initiate interaction without player involvement. A social encounter generally follows:

1. schedules, goals, or narration place characters within a valid interaction context;
2. perception establishes who notices whom;
3. eligible character engines propose intentions or actions;
4. a scene narrator sequences the exchange and environment;
5. participants independently interpret perceived behavior;
6. the engine validates and commits resulting events;
7. memories, beliefs, relationships, plans, and narrative threads update;
8. those changes influence future choices and encounters.

The engine SHOULD allow repeated interactions to create feedback loops: affinity may increase future contact, conflict may cause avoidance, gossip may change third-party beliefs, and commitments may modify schedules.

## 12. Narrator model

Narrators are scoped by authority and context. Initial narrator roles may include:

- **world narrator:** broad conditions and setting developments;
- **location narrator:** immediate environment, incidental activity, and local consequences;
- **scene narrator:** pacing, turn order, interruptions, and scene transitions;
- **social narrator:** unresolved tensions, opportunities, and relationship-relevant beats;
- **continuity narrator or auditor:** contradictions, duplicated entities, and implausible proposals.

Narrators MAY:

- select plausible coincidences;
- introduce local complications and minor characters;
- advance unresolved threads;
- summarize uneventful intervals;
- create opportunities for characters to act;
- propose environmental consequences.

Narrators MUST NOT:

- directly dictate a character's private feelings;
- reveal information unavailable to an observer;
- violate spatial, temporal, or access constraints;
- overwrite pinned facts;
- require a predetermined outcome;
- force player participation.

## 13. Time and scheduling

The engine SHOULD operate against real time when possible. Simulation time MUST nevertheless be explicit and controllable.

The scheduler SHOULD be event-driven. It MUST NOT require continuous model inference for every character. Resolution varies by relevance:

- active scenes may advance in seconds;
- nearby activity may advance in minutes;
- quiet intervals may be summarized;
- dormant characters may update only at consequential milestones.

After server downtime, catch-up SHOULD:

1. advance deterministic schedules;
2. resolve required commitments and necessities;
3. select a bounded number of consequential encounters;
4. update state through validated events;
5. summarize skipped uneventful time;
6. resume detailed simulation near the present.

## 14. Character-engine compatibility

### 14.1 Capability negotiation

Providers advertise capabilities rather than conforming to an LLM-specific interface. Possible capabilities include:

- profile completion;
- action selection;
- dialogue generation;
- perception interpretation;
- memory formation;
- relationship appraisal;
- planning;
- narration;
- structured output;
- streaming output;
- provider-owned persistent state.

The orchestrator MUST route only supported requests and MUST define fallbacks for optional capabilities.

### 14.2 Request envelope

A provider request SHOULD contain:

- request and world identifiers;
- capability requested;
- character or narrator identity;
- scoped canonical context;
- scoped subjective context;
- allowed actions or output schema;
- time and resource budget;
- privacy classification;
- idempotency key;
- provider-specific extension data.

### 14.3 Response envelope

A provider response SHOULD contain:

- proposed actions or structured content;
- confidence or uncertainty when supported;
- provider and engine version metadata;
- resource usage when available;
- source-specific state updates;
- validation warnings;
- human-readable output when applicable.

### 14.4 Isolation

Providers MUST receive only the context authorized for their role. A character engine MUST NOT automatically receive other characters' private memories, canonical secrets unknown to that character, unrelated imported source material, or user credentials.

Provider failure MUST degrade the affected capability rather than corrupting the world. Circuit breakers, timeouts, retries, and deterministic fallbacks SHOULD be supported.

## 15. Imports and world packs

The engine SHOULD ingest authorized source material from structured files, game exports, APIs, and wiki-like documents. Import proceeds through:

1. acquisition;
2. parsing and source retention;
3. claim extraction;
4. entity and alias resolution;
5. chronology and relationship resolution;
6. conflict detection;
7. user review when required;
8. world-pack assembly;
9. canonical import.

A world pack SHOULD contain entities, claims, relationships, chronology, provenance, source references, constraints, optional provider assignments, and unresolved conflicts.

Importers MUST NOT treat every sentence as objective truth. They SHOULD distinguish narrative statements, dialogue, speculation, alternate continuities, gameplay abstractions, and editorial commentary.

The project SHOULD facilitate lawful personal imports while avoiding unauthorized redistribution of copyrighted source text, artwork, or proprietary character data.

## 16. Persistence and storage

### 16.1 Requirements

Persistence MUST:

- run embedded in the self-hosted server;
- require no separately administered database service;
- support concurrent access by many server tasks within one process;
- provide durable atomic writes;
- provide consistent read snapshots;
- scale to large event, memory, relationship, and import indexes;
- support online checkpoints and recovery;
- preserve forward migration paths.

Only one authoritative process may open a world for mutation at a time. All clients and workers access it through the server.

### 16.2 Proposed storage design

RocksDB is the current proposed primary store because its embedded, multithreaded LSM design matches the expected read/write workload. This choice remains subject to implementation benchmarks.

The store should use explicit keyspaces or column families for:

- canonical entities;
- event ledger;
- relationships and reverse indexes;
- claims and beliefs;
- memories and retrieval indexes;
- spatial occupancy;
- scheduled work;
- social-horizon indexes;
- narrative threads;
- provider state references;
- import provenance.

Large immutable source documents and generated media SHOULD use a content-addressed blob store adjacent to the database. The primary store retains hashes, metadata, indexes, and references.

### 16.3 World directory

A portable world is expected to resemble:

```text
world/
  manifest.json
  database/
  blobs/
  imports/
  providers/
  backups/
```

## 17. Event commitment

Every consequential transition SHOULD be represented as an immutable event. Materialized current state and indexes may be updated in the same atomic transaction or write batch.

An event includes:

- stable identifier;
- world and simulation time;
- event type;
- participants and location;
- causal predecessors where applicable;
- objective payload;
- observer-specific perceptions or references to them;
- provenance;
- validation result;
- resulting state revision.

The event ledger enables auditing, reconstruction, debugging, migration, replay, and explanation of relationship or belief changes.

## 18. Validation and safety boundaries

Before commitment, proposals MUST be checked for:

- entity existence and lifecycle validity;
- spatial and temporal possibility;
- participant authority;
- knowledge and perception constraints;
- schema validity;
- pinned-fact contradictions;
- bounded numerical changes;
- duplicate or replayed requests;
- provider timeout or malformed output.

Administrative tooling SHOULD permit users to inspect provenance, pin facts, correct or retcon events, merge duplicate entities, regenerate uncommitted proposals, replace providers, and restore checkpoints.

## 19. Privacy and credentials

World data is private by default even though the engine is open source. Provider credentials MUST remain outside portable world exports and MUST NOT enter prompts, events, logs, or imported packs.

Each provider configuration SHOULD declare what data may leave the server. The engine SHOULD make external transmission visible and configurable by capability, character, world, and provider.

## 20. Performance model

Performance work should optimize for perceived continuity rather than maximum population count.

The engine SHOULD:

- avoid inference for routine deterministic behavior;
- batch compatible provider requests where supported;
- cache immutable source context;
- incrementally maintain graph and retrieval indexes;
- bound narrator consideration sets;
- apply backpressure to background simulation;
- prioritize player-visible scenes and time-sensitive commitments;
- measure promotion latency, event throughput, provider latency, storage growth, and catch-up time.

## 21. Initial reference scenario

The first reference world contains:

- one apartment building;
- several homes and shared spaces;
- a nearby street, café, shop, park, and workplace;
- walking routes and one transit route;
- approximately twelve materialized characters;
- additional referenced or stub characters beyond the active horizon;
- nearby speech, sound propagation, text messaging, and phone calls;
- real-time schedules and bounded offline catch-up;
- narrator-generated social opportunities;
- autonomous character interactions and relationship development.

The primary acceptance experiment runs this world for thirty simulated days and evaluates whether distinct friendships, avoidance patterns, rumors, conflicts, habits, and social groups emerge from traceable events.

## 22. Initial milestones

### Milestone 1: Persistent kernel

- World directory and manifest
- Embedded store abstraction
- Event ledger and atomic state projections
- Stable identifiers, provenance, and migrations
- Character, relationship, location, claim, and narrative-thread records

### Milestone 2: Scope and promotion

- Social graph and relevance scoring
- Lifecycle transitions
- Context assembly
- Deterministic promotion provider
- Promotion validation and audit events

### Milestone 3: Space and communication

- Location hierarchy and route graph
- Presence and scheduled movement
- Sound and basic visual perception
- Text and phone communication state machines

### Milestone 4: Narrators and character engines

- Capability-based provider protocol
- Narrator proposal lifecycle
- Character action and appraisal lifecycle
- At least one scripted provider
- At least one language-model adapter
- Timeouts, fallbacks, and privacy controls

### Milestone 5: Reference world and client

- Browser-based administrative and play client
- Reference apartment-block world
- Real-time and catch-up scheduler
- Thirty-day autonomous simulation harness
- Relationship, memory, and continuity diagnostics

### Milestone 6: Imports

- Import staging and provenance
- Structured world-pack format
- Text/wiki claim extraction adapter
- Entity-resolution review tools
- Export, backup, and portability documentation

## 23. Open decisions

The following remain intentionally unresolved:

- final implementation language and supported platforms;
- final embedded storage engine after benchmarks;
- exact relationship dimensions and update model;
- relevance scoring and default social-depth limits;
- narrator selection and arbitration policy;
- character-engine wire protocol;
- world-pack schema and versioning mechanism;
- whether deterministic replay is required across engine versions;
- licensing for the engine and compatibility SDKs;
- default content and provider safety policies;
- client protocol and offline client behavior.

## 24. Success criteria

CharacterGen Engine succeeds when:

1. characters from different engine sources can coexist behind one persistent world contract;
2. characters can meet without player orchestration and develop traceable, asymmetric relationships;
3. newly relevant characters can be promoted from accumulated context without contradicting established history;
4. narrators can advance the world while respecting space, time, knowledge, and character autonomy;
5. the simulation remains bounded as the implied population expands;
6. worlds survive provider replacement, server restart, backup, migration, and long periods of play;
7. the architecture can adopt future character technologies without replacing the canonical world model.
