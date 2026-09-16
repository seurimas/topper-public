# Skills Primer

This document explains how to read the **per-skill Markdown files** in the Aetolia skill knowledge base, and how to use them to make sense of a combat log or to write/review a behavior tree. That knowledge base is generated locally into `scripts/output/` by `scripts/scrape_skills.py` (see §2).

`scripts/output/` is gitignored — it's generated data, not checked into the repo. Run the scripts (see §2) to (re)populate it before relying on this primer.

(See also [BT_PRIMER.md](BT_PRIMER.md) for how this repo's behavior trees consume affliction/combat concepts in code.)

---

## 1. Scope: Only the Per-Skill `.md` Files

This primer, and the recommended workflow it describes, cover **only** the generated per-skill Markdown files:

```
scripts/output/General Skills/<Skill>.md
scripts/output/Mercantile Skills/<Skill>.md
scripts/output/Character Classes/<Class>/<Skill>.md
```

Each of these is a direct, mechanical rendering of the official in-game help text scraped from aetolia.com — nothing more. Treat these as the reliable reference for ability syntax, stated effects, and any documented stats.

**Do not use anything else as a source of truth**, especially not the hand-written notes living alongside the original scraper in `%USERPROFILE%\OneDrive\Cowork\Aetolia` (e.g. `*_combat_insights.md`, `CLARIFICATIONS_README.md`, `curing_bible.md`, `principles_of_combat.md`, `combat_reasoning.md`, `cure_timers_and_locks.md`, `venom_affliction_table.md`, `combat_log_analysis_system_spec.md`, `location_effects_and_room_objects.md`, `emotional_state_system.md`, `CLAUDE.md`, and similar files). The raw JSON files in `scripts/output/` (`aetolia_classes.json`, etc.) are also out of scope — same data as the `.md` files, just less convenient to read.

Those extra files are **unvetted** — they may be outdated, speculative, or simply wrong, and have not been checked against the current live game. If you find yourself tempted to cite one to explain a mechanic, stop and look for the same information in the relevant per-skill `.md` file instead. If it isn't stated there, treat it as unknown rather than filling the gap from an unvetted note.

---

## 2. Directory Layout & Regenerating

All paths below are relative to this repo (`topper/`):

```
scripts/scrape_skills.py               # scrapes aetolia.com -> writes scripts/output/*.json AND the per-skill .md files

scripts/output/General Skills/<Skill>.md              # abilities every character has, regardless of class
scripts/output/Mercantile Skills/<Skill>.md           # crafting/profession abilities (mostly non-combat)
scripts/output/Character Classes/<Class>/<Skill>.md   # the 2-3 skills unique to each class
```

`scripts/output/` is **gitignored** — it's regenerated, not committed. To (re)populate it:

```powershell
cd scripts
python scrape_skills.py              # general + mercantile -> JSON + General/Mercantile Skills .md files
python scrape_skills.py classes       # all classes -> JSON + Character Classes/ .md files (or pass specific class names)
```

Each call scrapes and writes both the JSON and the finished `.md` files in one pass — there's no separate generation step.

---

## 3. Anatomy of a Skill File

Every generated skill `.md` file (general, mercantile, or class) follows the same template, produced by `render_skill_markdown()` in `scripts/scrape_skills.py`:

```markdown
# <SkillName>
*<Category> Skill — Aetolia*

Source: https://www.aetolia.com/...

## Abilities Overview
| Ability | Summary                              |
| ------- | ------------------------------------ |
| ...     | one-line summary of each ability ... |

---

## Ability Details
### <AbilityName>
*<one-line summary>*

<full description text, copied from in-game HELP>

**Details:**
| Stat | Value |
| ---- | ----- |
| ...  | ...   | <- only present if the ability has a stats table (e.g. Base Divert Chance) |
```

Reading tips:
- The **Abilities Overview** table is a quick index — scan it first to find the ability you need, then jump to its full entry.
- The **description** text is verbatim in-game help text (HTML stripped). It uses in-game syntax conventions like `SYNTAX: COMMAND <ARG>`.
- The **Details** table (when present) is the only place numeric stats (chances, durations, costs) are officially stated. Most abilities don't have one — if a number isn't in this table, treat it as undocumented rather than guessing.
- "This ability is passive." in the description means it has no command syntax — it's always active/automatic.
- The file only tells you what an ability *does when used*. It does not tell you balance/equilibrium cost, cooldowns, or how it combines with other abilities unless the description explicitly says so — don't infer these from silence.

---

## 4. Using Skill Files to Read a Combat Log

A combat log is full of command names, spell/attack names, and affliction messages. To understand what actually happened in a log:

1. **Identify the actor's class** from context (or from `scripts/output/Character Classes/` folder names) so you know which skill files apply on top of `scripts/output/General Skills/`.
2. **For each unfamiliar action/command in the log**, find the matching ability name. Ability names in logs are usually the `<AbilityName>` used in-game (e.g. `dstab`, `envenom`, `songward`) — search the relevant class's skill `.md` files (and `General Skills/` for universal actions like weapon attacks) via the **Abilities Overview** table.
3. **Read the full entry** for that ability to learn its stated syntax and effect — this tells you what the actor was trying to accomplish (e.g. "Diversion" reduces damage from a physical attack; "Envenom" applies a venom to a weapon).
4. **Cross-reference affliction names** that appear in the log against the abilities that mention inflicting them, to figure out which action caused which affliction.
5. If the log shows behavior a skill file doesn't explain (e.g. a specific number, an interaction, a timing), **do not fill the gap from the unvetted notes** (§1). State plainly that the mechanism isn't documented in the vetted skill files.

---

## 5. Using Skill Files to Read or Write a Behavior Tree

Behavior trees in `behavior_trees/` (see [BT_PRIMER.md](BT_PRIMER.md)) reference ability names and afflictions in their actions and predicates. When reviewing or writing a tree:

1. **Look up every ability the tree acts on** in the class's skill `.md` files (or `scripts/output/General Skills/`) to confirm the ability actually exists, its syntax, and its stated effect before assuming the tree's intent.
2. **Verify affliction/predicate assumptions** (e.g. `AllAffs`, `AffStacksOver`) against what the relevant ability's description actually says it inflicts — don't assume a predicate is checking for the right thing just because the tree names it plausibly.
3. **When a stats table exists** (e.g. a chance or bonus value), use those numbers to sanity-check thresholds used in the tree (e.g. a `SubTree`/`Selector` branch that only makes sense above a certain success chance).
4. **When the skill file doesn't document something the tree relies on** (timing, stacking behavior, interaction with another ability), flag that as an assumption baked into the tree rather than a verified fact — do not resolve it by reading the unvetted notes in §1.

---

## 6. General vs. Mercantile vs. Class Skills

- **General Skills** (`scripts/output/General Skills/`): available to every character regardless of class (e.g. Weaponry, Avoidance, Survival, Persuasion). Combat-relevant baseline mechanics live here.
- **Mercantile Skills** (`scripts/output/Mercantile Skills/`): crafting/profession skills (Alchemy, Enchantment, Forging, Herbalism, Toxicology) — mostly non-combat, relevant for understanding item/venom sourcing.
- **Character Classes** (`scripts/output/Character Classes/<Class>/`): each class has exactly 2-3 class-specific skill files (e.g. Bard has Performance, Songcalling, Weaving) that define its unique combat identity, layered on top of General Skills. Ignore any other file present in these class folders (see §1).

---

## 7. Trust & Freshness Notes

- These `.md` files are a mechanical scrape of the live help text — they can go stale if aetolia.com changes an ability. If something looks inconsistent with observed behavior, treat the file as possibly outdated rather than wrong about mechanics it does describe.
- Never substitute information from the unvetted files (§1) to resolve a gap or contradiction — surface the uncertainty instead.

