# CharacterGen Engine

CharacterGen Engine is a persistent world engine for characters who can continue to grow after their initial creation. It is intended to support as many character-engine sources as possible—including game engines, prescripted characters, LLMs, and future technologies for dynamic character expansion.

The engine maintains a scoped, persistent world in which relevant characters occupy places, perceive events, interact through available communication channels, form memories and opinions, develop relationships, and become more fully simulated as they move closer to the player's life.

The project is currently in its specification phase. See [SPEC.md](SPEC.md) for the draft product and architecture specification.

## Status

- Public architectural testbed
- Self-hosted, single-user first
- Server/client design
- Implementation language and embedded persistence engine are proposed, not yet final

## Principles

- Characters exist in a world, not only in prompts.
- Narrators move the world forward without dictating every character's response.
- The simulation is scoped around relevance rather than attempting to model everyone.
- Established facts survive changes in models, providers, and presentation clients.
- Character sources are interchangeable behind explicit capability contracts.
- Imported material retains provenance and remains distinguishable from generated additions.

## License

No license has been selected yet. Until one is added, the repository is publicly visible but normal copyright restrictions apply.
