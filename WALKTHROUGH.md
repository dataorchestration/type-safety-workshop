# Workshop Walkthrough — Make Invalid Data Unrepresentable

**90 minutes | Hands-on | Zero dependencies (std library only)**

## Structure

Five exercises building from fundamentals to advanced. Phantom types moved to bonus material (too advanced for first workshop — available in `src/bonus/` for self-study).

| Time | Exercise | Pattern | The 2AM Incident | Why Not Possible Elsewhere |
|------|----------|---------|------------------|---------------------------|
| 0:00–0:10 | Opening | "The 2 AM Pipeline" story | Set the stakes | — |
| 0:10–0:25 | **Ex 1: Option** | Null safety via enum | Java NullPointerException crashed user service | Java/Python: null exists everywhere, no compile-time protection |
| 0:25–0:40 | **Ex 2: Result + Enums** | Typed errors, ? operator | 50-frame stack trace from uncaught exception | Java checked exceptions ignored. Python has no way to express "can fail" in signature |
| 0:40–0:55 | **Ex 3: Newtypes** | Domain-distinct types | CustomerId/OrderId swap in JOIN | Java: possible but heap+GC overhead. Python: hints are advisory |
| 0:55–1:15 | **Ex 4: Trait bounds** | Numeric-only ops | `df["name"].sum()` → "AliceBobCharlie" revenue | pandas silent concat. Spark runtime error |
| 1:15–1:25 | **Ex 5: Typestate (lite)** | Builder with state | Incomplete SQL query in production | Java builders: runtime null checks |
| 1:25–1:30 | Wrap-up + Claude skill | Tying it together | "If it compiles, it's correct" | — |

## Why This Order

Reviewers flagged phantom types as too advanced for a first workshop. They're right. The new order:

1. **Option** and **Result** are the foundation — every Rust program uses them. Without understanding enums-with-data, nothing else clicks.
2. **Newtypes** apply the "wrap for safety" idea to domain types
3. **Trait bounds** extend it to operations (not just types)
4. **Typestate** is the capstone — combines newtypes + enum states + trait methods

Phantom types require understanding all four + `PhantomData`. Saved for bonus material.

## Motivation Hook (60 seconds before each exercise)

- **Option:** "Who's been paged for a NullPointerException? We're killing that bug — not catching it, making it impossible to write."
- **Result:** "Java throws exceptions you didn't know about. Python raises anywhere. Rust signatures tell the truth — compiler enforces it."
- **Newtypes:** "You could do this in Java — nobody does because every wrapper costs heap + GC. In Rust it's zero bytes. No excuse."
- **Trait bounds:** "`df['name'].sum()` in pandas concatenates strings. We've all hit this. Today: compile error."
- **Typestate:** "Java builders pretend to be safe with runtime null checks in `.build()`. We make `.build()` literally not exist until complete."

## Deliverables in Repo

| Item | Location |
|------|----------|
| Setup scripts (mac/linux/win) | `scripts/setup.sh`, `scripts/setup.ps1` |
| 5 exercises + solutions | `src/exercises/`, `src/solutions/` |
| Bonus: phantom types | `src/bonus/` |
| Full pipeline demo | `src/full_pipeline.rs` |
| Claude Code skill | `skill/SKILL.md` |

## Key Design Decisions

- **Zero dependencies** — pure std library. No cargo download waits, no version conflicts.
- **Stories, not theory** — each exercise starts with a production incident.
- **Break-first learning** — write wrong code, read the error, learn the fix.
- **Phantom types deferred** — available in `src/bonus/` for participants ready for more depth.
