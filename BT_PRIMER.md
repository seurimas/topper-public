# Behavior Tree Primer

This document provides all the context needed to read and write behavior trees (BTs) for topper. BTs are JSON files in `behavior_trees/` organized by class. The engine lives in `topper-aetolia/src/bt_match/` and `topper-aetolia/src/bt/`.

This primer is **general-purpose** -- it covers the full system, not just one class. Use it to write BTs for any class, including new ones.

---

## 1. How the Engine Works

1. `MatchRunner` loads a tree by name via `get_tree("class/subtree_name")`, which reads `behavior_trees/class/subtree_name.json`.
2. The tree is traversed depth-first. Each node returns **Complete** (success) or **Failed** (failure).
3. **Actions** append commands to `controller.plan` (the queued action list).
4. **Predicates** test conditions against the game model (timeline state).
5. The engine compares planned actions against observed combat log to detect divergences.

Trees are stateless between ticks -- all persistent state lives in the `BehaviorController`:
- `plan`: The queued action list.
- `target`: Current opponent name.
- `aff_priorities`: Venom priority stack (`Vec<VenomPlan>`).
- `plan_hints`: Key-value string store for cross-subtree coordination.
- `plan_tags`: Tags set during execution.
- `first_aid_settings`: Healing configuration.
- `nonce`: Incrementing counter for deterministic randomization.

---

## 2. Control Nodes

These are the structural building blocks. Every BT is composed of these.

| Node | JSON | Behavior |
|------|------|----------|
| **Selector** | `{"Selector": [...]}` | Tries children left-to-right. Succeeds on the **first** child that succeeds. (OR logic) |
| **Sequence** | `{"Sequence": [...]}` | Runs children left-to-right. Fails on the **first** child that fails. (AND logic) |
| **Inverter** | `{"Inverter": {...}}` | Wraps one child. Flips Complete <-> Failed. (NOT) |
| **Succeeder** | `{"Succeeder": {...}}` | Wraps one child. Always returns Complete. |
| **Failer** | `{"Failer": {...}}` | Wraps one child. Always returns Failed. |
| **SubTree** | `{"SubTree": "path/name"}` | Loads and executes `behavior_trees/path/name.json`. |

**Key insight:** Selector = "try until something works." Sequence = "do all of these, bail on first failure." Almost every BT pattern is built from nesting these two.

---

## 3. Targets

Most predicates and actions take an `AetTarget`:

- `"Me"` -- The player running the tree.
- `"Target"` -- The current combat opponent.

---

## 4. Predicates (Conditions)

Predicates return Complete (true) or Failed (false). They never modify state.

### 4.1 Affliction Predicates

These are the workhorses of strategy logic.

| Predicate | JSON Example | Meaning |
|-----------|-------------|---------|
| **`AllAffs`** | `{"AllAffs": ["Target", ["Asthma", "Slickness"]]}` | Target has **every** listed affliction |
| **`SomeAffs`** | `{"SomeAffs": ["Target", ["Paralysis", "Paresis"]]}` | Target has **at least one** |
| **`NoAffs`** | `{"NoAffs": ["Target", ["Rebounding", "AssumedRebounding"]]}` | Target has **none** |
| `IsAffectedBy` | `{"IsAffectedBy": ["Target", "Clumsiness"]}` | Single-aff alternative to SomeAffs |
| `AffCountOver` | `{"AffCountOver": ["Target", 3, ["Asthma", "Anorexia", "Slickness", "Paresis"]]}` | Count of listed affs >= N |
| `AffCountUnder` | `{"AffCountUnder": ["Target", 2, [...]]}` | Count <= N |
| `AffCountEqual` | `{"AffCountEqual": ["Target", 1, [...]]}` | Count == N |
| `AffStacksOver` | `{"AffStacksOver": ["Target", 2, "Ablaze"]}` | Stacks of specific aff >= N |
| `RandomCuresOver` | `{"RandomCuresOver": ["Target", 5]}` | Random-curable affs >= N |
| `MentalAffsOver` | `{"MentalAffsOver": ["Target", 3]}` | Mental affs >= N |
| `IsProne` | `{"IsProne": "Target"}` | Target is prone |
| **`Buffered`** | `{"Buffered": ["Target", "Asthma"]}` | Aff present with cure depth > 1 (survives one cure) |

### 4.2 Lock Predicates

| Predicate | JSON Example | Meaning |
|-----------|-------------|---------|
| **`NearLocked`** | `{"NearLocked": ["Target", "Soft", 2]}` | Target needs <= N more affs for lock |
| `Locked` | `{"Locked": ["Target", true]}` | Target is locked (`true` = hard lock >= 10s) |
| `CannotCure` | `{"CannotCure": ["Target", "Paresis"]}` | Aff too deep for target to cure in time |
| `PriorityAffIs` | `{"PriorityAffIs": ["Target", "Asthma"]}` | Target's most urgent cure is this aff |

**Lock types:** `Soft`, `Buffered`, `Hard`, `HardVenom`

### 4.3 Limb Predicates

All limb predicates use a **LimbDescriptor** (see Section 6).

| Predicate | JSON Example | Meaning |
|-----------|-------------|---------|
| `CanBreak` | `{"CanBreak": ["Target", {"Static": "LeftLegDamage"}, 30.0]}` | Breakable in 1 hit with given damage |
| `RestoredBreak` | `{"RestoredBreak": ["Target", {"Static": "LeftLegDamage"}, 30.0]}` | Same, but assumes restoration first |
| `CanMangled` | `{"CanMangled": ["Target", {"Static": "LeftArmDamage"}, 25.0]}` | Mangleable in 1 hit |
| `RestoredMangle` | Same pattern | Assumes restoration |
| **`CanMend`** | `{"CanMend": ["Target", {"Static": "LeftLegDamage"}]}` | Limb is crippled but not broken (mendable) |
| `LimbOver` | `{"LimbOver": ["Target", {"Static": "TorsoDamage"}, 50.0, true]}` | Damage > threshold (last arg: assume restoration) |
| `LimbsOver` | `{"LimbsOver": ["Target", [...descriptors], 100.0, false]}` | Total damage across limbs > threshold |
| `IsRestoring` | `{"IsRestoring": ["Target", {"Static": "LeftLegDamage"}]}` | Limb actively restoring |
| `IsRestoringAny` | `{"IsRestoringAny": "Target"}` | Any limb restoring |
| `IsOverRestoring` | `{"IsOverRestoring": ["Target", {"Static": "LeftLegDamage"}]}` | Restoring with <= 33% damage (wasteful) |
| `LimbsEqual` | `{"LimbsEqual": ["Target", desc_a, desc_b]}` | Two limbs have same damage |
| `AtLeastNLimbsOver` | `{"AtLeastNLimbsOver": ["Target", [descs...], 2, 40.0, true]}` | N limbs exceed threshold |
| `LimbsBreakableCount` | See below | Complex: per-limb damage breakability |

**LimbsBreakableCount** uses a struct (not an array):
```json
{
    "LimbsBreakableCount": {
        "target": "Target",
        "head_damage": 6.0,
        "torso_damage": 8.5,
        "left_arm_damage": 9.5,
        "right_arm_damage": 9.5,
        "left_leg_damage": 9.0,
        "right_leg_damage": 9.0,
        "min_count": 3,
        "assume_restoration": true
    }
}
```

### 4.4 Balance & Timing

| Predicate | JSON Example | Meaning |
|-----------|-------------|---------|
| **`HasBalanceEquilibrium`** | `{"HasBalanceEquilibrium": "Me"}` | Both bal + eq available |
| `HasBalance` | `{"HasBalance": "Me"}` | Balance available (< 0.15s) |
| `HasEquilibrium` | `{"HasEquilibrium": "Me"}` | Equilibrium available |
| `HasHandBalance` | `{"HasHandBalance": "Me"}` | Either hand balance available |
| `BalanceUnder` | `{"BalanceUnder": ["Target", "ClassCure1", 2.0]}` | Balance timer <= threshold |
| `BalanceOver` | `{"BalanceOver": ["Me", "Equilibrium", 1.5]}` | Balance timer > threshold |
| `HasTree` | `{"HasTree": ["Target", 2.5]}` | Tree tattoo available (buffer in seconds) |
| `HasFocus` | `{"HasFocus": ["Target", 2.5]}` | Focus available (buffer in seconds) |
| `HasFitness` | `{"HasFitness": ["Target", 1.0]}` | Fitness available |
| `HasClassCure` | `{"HasClassCure": ["Target", 1.0]}` | Class-specific cure available |
| **`ReboundingWindow`** | `{"ReboundingWindow": ["Target", 150]}` | Rebounding timer > threshold (ms) |
| **`ReboundingComing`** | `{"ReboundingComing": ["Target", 150]}` | Rebounding returning in <= threshold (ms) |
| `SalveBlocked` | `{"SalveBlocked": ["Target", 500]}` | Salve blocked for > threshold (ms) |

### 4.5 Channels & Combat State

| Predicate | JSON Example | Meaning |
|-----------|-------------|---------|
| `Channeling` | `{"Channeling": ["Target", "Death"]}` | Channeling specific type (or `null` for any) |
| `ChannelStoppedBy` | `{"ChannelStoppedBy": ["Target", "Paralysis"]}` | Channel interrupted by this aff |
| `CanDodge` | `{"CanDodge": "Target"}` | Can dodge attacks |
| `CanParry` | `{"CanParry": "Target"}` | Can parry |
| `KnownParry` | `{"KnownParry": ["Target", {"Static": "HeadDamage"}]}` | Known parry location |
| `ExpectedParry` | `{"ExpectedParry": ["Target", {"Static": "TorsoDamage"}]}` | Predicted parry location |

### 4.6 Miscellaneous

| Predicate | JSON Example | Meaning |
|-----------|-------------|---------|
| **`NonceModEqual`** | `{"NonceModEqual": {"modulus": 6, "remainder": 0}}` | Deterministic cycling (nonce % mod == rem) |
| `Persuading` | `{"Persuading": "Me"}` | Being persuaded |
| `StatUnderPercent` | `{"StatUnderPercent": ["Health", "Target", 30.0]}` | Stat below percentage |
| `IsGrounded` | `{"IsGrounded": "Target"}` | At ground level |
| `IsFlying` | `{"IsFlying": "Target"}` | Flying |
| `IsClimbing` | `{"IsClimbing": "Target"}` | In trees/on roof |
| `RoomIsTagged` | `{"RoomIsTagged": "outdoors"}` | Room has tag |
| `ClassIn` | `{"ClassIn": ["Target", ["Sentinel", "Predator"]]}` | Target is one of these classes |
| `PipeEmpty` | `{"PipeEmpty": ["Target", "elm"]}` | Named pipe empty |

### 4.7 Hint Predicates

| Predicate | JSON Example | Meaning |
|-----------|-------------|---------|
| **`HintSet`** | `{"HintSet": ["PRIMARY_ELEMENT", "FIRE"]}` | Hint matches value |
| `LimbHintIs` | `{"LimbHintIs": ["target_limb", "LeftLegDamage"]}` | Hint resolves to limb |

### 4.8 Class-Specific Predicates

Class predicates are wrapped in a class tag:

```json
{"SentinelPredicate": ["Target", "HasColdResin"]}
{"BardPredicate": ["Me", "InRhythm"]}
{"PredatorPredicate": ["Me", "HasOrgyuk"]}
{"AscendrilPredicate": ["Me", "AfterburnRaising"]}
{"ZealotPredicate": ["Me", "ZenithUp"]}
{"InfiltratorPredicate": ["Target", "Sealed"]}
{"SiderealistPredicate": ["Target", {"VibrationInRoom": "Enigma"}]}
```

For predicates with arguments, the predicate itself is a JSON object:
```json
{"BardPredicate": ["Target", {"Needled": null}]}
{"BardPredicate": ["Target", {"HalfbeatWithin": 0.5}]}
{"SiderealistPredicate": ["Target", {"VibrationDormant": "Enigma"}]}
```

---

## 5. Actions

Actions queue commands and return Complete (queued successfully) or Failed (preconditions not met).

### 5.1 Common Actions (All Classes)

| Action | JSON Example | Effect |
|--------|-------------|--------|
| **`UnstackAffs`** | `{"UnstackAffs": ["Nimbleness", "Insomnia"]}` | Remove affs from venom priority stack |
| **`PushAff`** | `{"PushAff": "Slickness"}` | Push aff to front of priority stack |
| **`PushLockers`** | `{"PushLockers": "Soft"}` | Push lock affs to front of stack |
| `PlainQebBehavior` | `{"PlainQebBehavior": "stand"}` | Queue raw command string |
| `TouchHammer` | `{"TouchHammer": "Target"}` | Touch hammer on shielded target |
| **`TagPlan`** | `{"TagPlan": "using_fire"}` | Tag current plan for tracking |
| **`HintPlan`** | `{"HintPlan": ["PRIMARY_ELEMENT", "FIRE"]}` | Store hint for cross-subtree use |
| **`CopyHint`** | `{"CopyHint": ["FAST_WEAPON", "WEAPON"]}` | Copy hint value from one key to another |
| **`SetLimbHint`** | `{"SetLimbHint": ["Target", {"Highest": [...]}, "LIMB"]}` | Resolve limb descriptor, store as hint |

### 5.2 Defense Actions

These appear as bare strings:

```json
"Parry"
"Repipe"
"Fitness"
"Dodge"
```

### 5.3 First Aid Actions

| Action | JSON Example | Effect |
|--------|-------------|--------|
| `AddFirstAidSettings` | See below | Configure healing priorities |
| `ResetFirstAidPriorities` | `"ResetFirstAidPriorities"` | Clear first aid settings |

**FirstAidSetting variants:**
```json
{"AddFirstAidSettings": [
    {"SimplePriority": ["Rebounding", 10]},
    {"SimplePriority": ["Insomnia", 8]},
    {"VitalsPriority": "Hp"},
    {"TreeCount": 1},
    {"FocusCount": 1},
    {"UseClotting": "On"},
    {"UseFocus": true},
    {"UseTree": true},
    {"ClotAbovePercentMana": 40},
    {"ClotAboveBleed": 50},
    "ResetPriorities"
]}
```

### 5.4 Enchantment Actions

```json
{"Pestilence": "Target"}
```

---

## 6. LimbDescriptor

Flexible limb selection used in limb predicates and `SetLimbHint`. Limb types: `HeadDamage`, `TorsoDamage`, `LeftArmDamage`, `RightArmDamage`, `LeftLegDamage`, `RightLegDamage`.

| Variant | JSON | Meaning |
|---------|------|---------|
| **`Static`** | `{"Static": "LeftLegDamage"}` | Fixed limb |
| **`Highest`** | `{"Highest": ["LeftArmDamage", "RightArmDamage"]}` | Highest damage from list |
| **`Lowest`** | `{"Lowest": ["LeftLegDamage", "RightLegDamage"]}` | Lowest damage from list |
| `HighestOver` | `{"HighestOver": [["LeftLegDamage", "RightLegDamage"], 30.0]}` | Highest limb exceeding threshold |
| `LowestOver` | `{"LowestOver": [["LeftLegDamage", "RightLegDamage"], 20.0]}` | Lowest exceeding threshold |
| `NotRestoring` | `{"NotRestoring": ["LeftLegDamage", "RightLegDamage"]}` | First non-restoring limb |
| `Breakable` | `{"Breakable": [["LeftLegDamage", 30.0], ["RightLegDamage", 30.0]]}` | First breakable at given damage |
| `Random` | `{"Random": ["LeftArmDamage", "RightArmDamage"]}` | Random from list |
| **`FromHint`** | `{"FromHint": "target_limb"}` | Resolve from stored hint |

---

## 7. Wrappers

Wrappers modify the environment for an entire subtree.

### WithoutAffsInStack

Temporarily removes afflictions from the venom priority stack, executes the wrapped tree, then restores the original stack. Top-level array format:

```json
[
    {"WithoutAffsInStack": ["Paresis", "Shyness"]},
    [
        {
            "Sequence": [
                {"BardPredicate": ["Me", "InRhythm"]},
                {"VenomAttack": "Needle"}
            ]
        }
    ]
]
```

### BardWrapper

Bard-specific wrapper that handles weapon wielding and thurible management. Applied to the top-level bard tree:

```json
[
    "BardWrapper",
    [
        {"Selector": [...]}
    ]
]
```

### Executor (FirstAid trees)

Used in first-aid trees. Runs all children and collects settings:

```json
{
    "Executor": [
        {"SubTree": "firstaid/common/default_defenses"},
        {"AddFirstAidSettings": [{"SimplePriority": ["Rebounding", 10]}]}
    ]
}
```

---

## 8. Patterns and Idioms

### Pattern 1: Guard-then-Act (Sequence)

The fundamental pattern. Predicates gate an action:

```json
{
    "Sequence": [
        {"AllAffs": ["Target", ["Arrhythmia"]]},
        {"SentinelComboFull": ["Target", "Slam", "Gouge"]}
    ]
}
```
*"If target has Arrhythmia, use Slam+Gouge."*

### Pattern 2: Priority Ladder (Selector of Sequences)

A Selector where each child is a guarded Sequence. First match wins. This is the single most important pattern -- it implements prioritized decision-making:

```json
{
    "Selector": [
        {
            "Sequence": [
                {"AllAffs": ["Target", ["Shielded", "Rebounding"]]},
                {"SentinelDualraze": "Target"}
            ]
        },
        {
            "Sequence": [
                {"SomeAffs": ["Target", ["Rebounding", "Shielded"]]},
                {"SentinelComboFull": ["Target", "Reave", "Stab"]}
            ]
        },
        {"SentinelCombo": "Target"}
    ]
}
```
*"Dualraze if both defenses up. Raze-combo if either. Otherwise normal combo."*

### Pattern 3: Negated Check (Inverter)

Negate a predicate to create "if NOT" conditions:

```json
{
    "Sequence": [
        {"Inverter": {"SentinelPredicate": ["Target", "HasColdResin"]}},
        {"Hurl": ["Target", "Lysirine"]}
    ]
}
```
*"If target does NOT have cold resin, hurl Lysirine."*

### Pattern 4: Sub-Tree Composition

Break strategies into named files and compose them in a priority Selector. This is how every class's main tree is structured:

```json
{
    "Selector": [
        {"SubTree": "sentinel/setup_beasts"},
        {"SubTree": "sentinel/spinecut"},
        {"SubTree": "sentinel/dualraze"},
        {"SubTree": "sentinel/mentals_combo"},
        {"SubTree": "sentinel/apply_resin"},
        {"SubTree": "sentinel/secure_locks"},
        {"SentinelFirstStrike": ["Target", "Slash"]}
    ]
}
```
*"Try each strategy in priority order. Fall through to basic Slash."*

### Pattern 5: Side-Effects with Failer

Use `Failer` to execute actions without short-circuiting the parent Selector. The action runs but always "fails," so the Selector continues:

```json
{
    "Selector": [
        {"Failer": {"TempoVenom": true}},
        {"Failer": {"CopyHint": ["FAST_WEAPON", "WEAPON"]}},
        {"SubTree": "bard/base_strategy"}
    ]
}
```
*"Set tempo venom and copy hint (side effects), then continue to main strategy."*

This is also how the `defend_first` tree works -- defensive actions always run, never stop the parent:

```json
{"Failer": {
    "Sequence": [
        {"PlainQebBehavior": "stand"},
        "Parry",
        "Repipe",
        "Fitness"
    ]
}}
```

### Pattern 6: Side-Effects with Succeeder

Use `Succeeder` to run a subtree that might fail without stopping a parent Sequence:

```json
{
    "Sequence": [
        {"PredatorPredicate": ["Me", "HasOrgyuk"]},
        {"Succeeder": {"SubTree": "predator/orgyuk/base"}}
    ]
}
```
*"If I have Orgyuk, try the orgyuk strategy (but don't fail the sequence if it doesn't find anything to do)."*

### Pattern 7: Balance Gating

The standard pattern for "don't try offense without balance":

```json
{
    "Selector": [
        {"SubTree": "class/follow_up"},
        {"Inverter": {"HasBalanceEquilibrium": "Me"}},
        {"SomeAffs": ["Me", ["Stun", "Asleep"]]},
        {"SubTree": "defend_first"},
        {"SomeAffs": ["Me", ["Paralysis"]]},
        {"SubTree": "class/main_offense"}
    ]
}
```

**How this works in a Selector:**
1. Try follow-ups first (may succeed without balance).
2. `Inverter` + `HasBalanceEquilibrium`: If I **don't** have bal+eq, this succeeds and the Selector stops. No offense attempted.
3. If stunned/asleep, the SomeAffs succeeds and the Selector stops.
4. `defend_first` always returns Failed (it's wrapped in Failer), so defensive actions run but the Selector continues.
5. If paralyzed, stop.
6. Otherwise, run main offense.

### Pattern 8: Lock Advancement

Push lock afflictions when close to securing a lock:

```json
{
    "Sequence": [
        {"NoAffs": ["Target", ["Rebounding", "AssumedRebounding"]]},
        {"Selector": [
            {
                "Sequence": [
                    {"Inverter": {"HasFocus": ["Target", 2.5]}},
                    {"Inverter": {"HasTree": ["Target", 0.0]}},
                    {"NearLocked": ["Target", "Soft", 2]},
                    {"PushLockers": "Soft"}
                ]
            },
            {
                "Sequence": [
                    {"NearLocked": ["Target", "Buffered", 2]},
                    {"PushLockers": "Buffered"}
                ]
            }
        ]},
        {"Twinshot": "Target"}
    ]
}
```
*"No rebounding? Check cure availability. If focus and tree are down and we're 2 affs from soft lock, push soft lockers. Else if near buffered lock, push those. Then attack."*

### Pattern 9: Nonce-Based Rotation

Use `NonceModEqual` for deterministic cycling through options. The nonce increments each BT tick, so modulo distributes evenly:

```json
{
    "Selector": [
        {
            "Sequence": [
                {"NonceModEqual": {"modulus": 6, "remainder": 0}},
                {"AdjustPriority": [["PummelLeft"], 1]}
            ]
        },
        {
            "Sequence": [
                {"NonceModEqual": {"modulus": 6, "remainder": 1}},
                {"AdjustPriority": [["Clawtwist"], 1]}
            ]
        },
        {
            "Sequence": [
                {"NonceModEqual": {"modulus": 6, "remainder": 2}},
                {"AdjustPriority": [["WanekickLeft"], 1]}
            ]
        }
    ]
}
```
*"Cycle through 6 attack priority boosts, one per tick."*

### Pattern 10: Channel Interrupt

```json
{
    "Selector": [
        {
            "Sequence": [
                {"Channeling": ["Target", "Death"]},
                {"PerformanceAttack": {"TempoThree": ["delphinium", "delphinium", "delphinium"]}}
            ]
        }
    ]
}
```
*"If target is channeling Death, interrupt with triple-delphinium tempo."*

### Pattern 11: Limb Targeting with Guards

Check limb state before committing to limb-specific attacks. Try each limb in a Selector:

```json
{
    "Selector": [
        {
            "Sequence": [
                {"CanMend": ["Target", {"Static": "LeftLegDamage"}]},
                {"Inverter": {"IsRestoring": ["Target", {"Static": "LeftLegDamage"}]}},
                {"SentinelPierce": ["Target", "left"]}
            ]
        },
        {
            "Sequence": [
                {"CanMend": ["Target", {"Static": "RightLegDamage"}]},
                {"Inverter": {"IsRestoring": ["Target", {"Static": "RightLegDamage"}]}},
                {"SentinelPierce": ["Target", "right"]}
            ]
        }
    ]
}
```
*"Pierce a mendable leg that isn't being restored. Try left, then right."*

### Pattern 12: Hint-Driven Element/Mode Selection

Set a hint early, then read it in downstream subtrees. Ascendril uses this for element cycling:

**In `ascendril/fire/primary.json`:**
```json
{
    "Sequence": [
        {"HintSet": ["PRIMARY_ELEMENT", "FIRE"]},
        {"Selector": [
            {"SubTree": "ascendril/brands/fire_air"},
            {"SubTree": "ascendril/fire/big_pyroclast"},
            {"SubTree": "ascendril/gated_attacks/spark"}
        ]}
    ]
}
```

**In a downstream subtree checking element:**
```json
{"HintSet": ["PRIMARY_ELEMENT", "FIRE"]}
```
*(As a predicate, HintSet returns Complete if the hint matches.)*

### Pattern 13: Stepped Resource Application

Apply resources in order, each step gated on the previous:

```json
{
    "Selector": [
        {
            "Sequence": [
                {"Inverter": {"SentinelPredicate": ["Target", "HasColdResin"]}},
                {"Inverter": {"SentinelPredicate": ["Target", "IsBurning"]}},
                {"Hurl": ["Target", "Lysirine"]}
            ]
        },
        {
            "Sequence": [
                {"SentinelPredicate": ["Target", "HasColdResin"]},
                {"Inverter": {"SentinelPredicate": ["Target", "HasHotResin"]}},
                {"Inverter": {"SentinelPredicate": ["Target", "IsBurning"]}},
                {"Hurl": ["Target", "Harimel"]}
            ]
        },
        {
            "Sequence": [
                {"SentinelPredicate": ["Target", "HasColdResin"]},
                {"Inverter": {"SentinelPredicate": ["Target", "IsBurning"]}},
                {"Combust": "Target"}
            ]
        }
    ]
}
```
*"Step 1: apply cold resin. Step 2: apply hot resin. Step 3: combust."*

### Pattern 14: Combo Solver (Predator)

Predator uses a combo solver system. Actions build an attack pool, then the solver picks the best combo:

```json
{
    "Selector": [
        {"SubTree": "predator/venom/add_venom_attacks"},
        {"SubTree": "predator/common/add_raze"},
        {"Failer": {"CalculateCombos": "Target"}},
        {"AffRateCombo": ["Target",
            [{"MinimumAttacks": 2}, {"EndsInStance": "VaeSant"}, {"WithAttack": "Veinrip"}],
            []
        ]},
        {"AffRateCombo": ["Target",
            [{"MinimumAttacks": 2}, {"WithAttack": "Vertical"}],
            []
        ]},
        {"Dartshot": "Target"}
    ]
}
```
*"Add attacks to pool. Calculate combos (side effect via Failer). Try best combo with Veinrip. Try with Vertical. Fall back to Dartshot."*

### Pattern 15: Priority-Based Combo Building (Zealot)

Zealot builds combos by adding attacks at priorities, then taking the top ones:

```json
{
    "Selector": [
        {"SubTree": "zealot/in_combo_damages/damage_any"},
        {"Failer": {
            "Sequence": [
                {"AllAffs": ["Me", ["Firefist"]]},
                {"SubTree": "zealot/adjust_for_firefist"}
            ]
        }},
        {"Failer": {
            "Sequence": [
                {"NoAffs": ["Target", ["SoreWrist"]]},
                "ToggleWristlashOnCombo"
            ]
        }},
        {"TakeComboAttacksIfOver": ["Target", 30]},
        {"TakeComboAttacks": "Target"}
    ]
}
```
*"Add damage attacks. Optionally adjust for firefist (side effect). Toggle wristlash if no sore wrist (side effect). Take combo attacks if priorities high enough, otherwise take whatever we have."*

### Pattern 16: Compound Logic (Inverter + Selector = NOR)

Negate a Selector to create "none of these are true":

```json
{
    "Sequence": [
        {"Inverter": {
            "Selector": [
                {"NoAffs": ["Target", ["Perplexity", "Misery", "Hollow"]]},
                {"BardPredicate": ["Target", "IronCollared"]}
            ]
        }},
        {"SubTree": "bard/upfront_essentials/awaken_first"}
    ]
}
```
*"If it's NOT the case that (target has no emotion affs OR target is iron-collared), then awaken."*

### Pattern 17: Curing Window Exploitation

Check that the target can't cure an affliction before applying the next step:

```json
{
    "Sequence": [
        {"CannotCure": ["Target", "Paresis"]},
        {"VenomAttack": "TempoOne"}
    ]
}
```
*"Only apply tempo venom when target can't cure Paresis in time."*

---

## 9. Common Associations in the Codebase

These associations appear repeatedly in existing trees and inform how to structure conditions:

- **Rebounding / AssumedRebounding / Shielded** -- Trees frequently check `NoAffs` for these before attacking, and use raze attacks (e.g., `Reave`, `SentinelDualraze`) to strip them. Many actions internally fail if these are present.
- **Stun / Asleep** -- Checked on `Me` to abort offense (you can't act while stunned/asleep).
- **Paralysis** -- Checked on `Me` as a softer abort (may still try some actions).
- **Tree / Focus** -- `HasTree` and `HasFocus` on `Target` inform lock timing. Locks are pushed when cures are down.
- **Rebounding timing** -- `ReboundingComing` and `ReboundingWindow` are used to time attacks around rebounding cycles (e.g., start a whirl before rebounding comes up).

---

## 10. File Organization

```
behavior_trees/
  defend_first.json                    # Shared defensive pre-checks (Failer-wrapped)
  firstaid/
    classless.json                     # Base first aid (Executor)
    classes/{class}.json               # Per-class first aid overrides
    common/default_defenses.json       # Shared defense priorities
  {class}/
    base.json                          # Entry point for the class
    base_balance.json                  # Main offense (when bal+eq available)
    follow_up/base.json                # Follow-up attack router
    follow_up/{strategy}.json          # Specific follow-ups
    {strategy}.json                    # Individual strategy sub-trees
    {category}/{strategy}.json         # Deeper nesting for complex classes
```

**Conventions:**
- `base.json` is always the entry point.
- Sub-trees are named for what they do.
- Deeper nesting groups related strategies (e.g., `bard/tempoing/`, `predator/venom/`, `zealot/in_combo_damages/`).
- Each sub-tree file should be focused on one decision or action.

---

## 11. bt_match.json Configuration

The `bt_match.json` file at the project root configures the match engine:

```json
{
    "ignore": ["Passive Order", "Focus", "Tree"],
    "skill_traps": {
        "CallCoyote": [
            {"Proc": {"caster": "LOG_ME", "category": "Woodlore", "skill": "Calling", ...}},
            {"Proc": {"caster": "", "category": "Woodlore", "skill": "Called", "annotation": "coyote", ...}},
            {"Proc": {"caster": "LOG_ME", "category": "TimelineFix", "skill": "Balance", ...}}
        ]
    }
}
```

- **`ignore`**: Skills to skip during divergence comparison.
- **`skill_traps`**: Simulated observation events injected when the engine encounters a specific action. Used to simulate game state changes (e.g., beast summoning updates the timeline).

---

## 12. Writing a New Class

To add behavior trees for a new class:

1. **Create the Rust side** in `topper-aetolia/src/classes/{class}/`:
   - `behavior.rs`: Define `{Class}Behavior` enum with all actions.
   - `predicate.rs`: Define `{Class}Predicate` enum with all conditions.
   - Register them in the parent `AetBehavior`/`AetPredicate` enums.

2. **Create `behavior_trees/{class}/base.json`** as the entry point. Follow the standard structure:
   ```json
   {
       "Selector": [
           {"Inverter": {"HasBalanceEquilibrium": "Me"}},
           {"SomeAffs": ["Me", ["Stun", "Asleep"]]},
           {"SubTree": "defend_first"},
           {"SomeAffs": ["Me", ["Paralysis"]]},
           {"SubTree": "{class}/main_offense"}
       ]
   }
   ```

3. **Create sub-trees** for each strategy, following the patterns above.

4. **Create `behavior_trees/firstaid/classes/{class}.json`** for healing configuration.

---

## 13. Worked Example: Adding a Beguile Follow-Up

**Task:** Add a sub-tree to sentinel's follow-up base to use Beguile to deliver Slickness if the target has buffered Asthma and any Paresis.

**Step 1:** Create `behavior_trees/sentinel/follow_up/beguile_slickness.json`:

```json
{
    "Sequence": [
        {"Buffered": ["Target", "Asthma"]},
        {"SomeAffs": ["Target", ["Paralysis", "Paresis"]]},
        {"PushAff": "Slickness"},
        {"SentinelSecondStrike": ["Target", "Beguile"]}
    ]
}
```

**Step 2:** Wire it into `behavior_trees/sentinel/follow_up/base.json`:

```json
{
    "Sequence": [
        {"SentinelPredicate": ["Me", "HasFirstStrike"]},
        {"Selector": [
            {"SubTree": "sentinel/follow_up/stick_arrythmia"},
            {"SubTree": "sentinel/follow_up/beguile_slickness"},
            {"SentinelSecondStrike": ["Target", "Flourish"]}
        ]}
    ]
}
```

The new subtree slots into the Selector's priority list between existing options and the default fallback.
