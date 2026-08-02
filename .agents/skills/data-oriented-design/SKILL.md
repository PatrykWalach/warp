---
name: data-oriented-design
description: "Shrink a struct's memory footprint: apply data-oriented design (DoD) when the user wants smaller structs, less memory use, fewer cache misses, or a faster hot path, or when they mention a layout trick — struct-of-arrays, storing fields out of band, encodings instead of polymorphism, indexes instead of pointers."
---

# Shrink the footprint

The CPU is fast and main memory is slow — an order of magnitude slower per cache level (L1 → L2 → L3 → RAM). Every memory access travels through a **cache line** (typically 64 bytes): access two things that live in the same line and it's free; reach into a second line and you may **evict** one and eat a **cache miss**. The whole game is fewer cache misses.

**Do more math, do less memory.** Reading memory is the bottleneck, not computing — a multiply is cheaper than an L1 read. Recompute derived data (line/column, a token's end position) rather than storing it; memoizing math is usually the wrong move. A kernel call from a heap allocation (`malloc`) is among the slowest things a CPU can do; keep it out of hot paths.

## The loop

1. **Pick the victim** — the struct with the most instances in memory (a game's monster, a compiler's token). Its **footprint** (bytes × count) is what you shrink.
   Completion: you can name the struct, its byte size, and roughly how many are alive at once.

2. **Shrink it.** Walk the five strategies; apply every one that fits and reject each that doesn't, with a reason in hand.
   Completion: every strategy below is either applied or explicitly rejected with a justification.

3. **Measure.** Wall-clock a real workload before and after, one change at a time.
   Completion: before/after numbers on the table and you can say which strategy earned which fraction.

Then repeat from step 1 on the next-biggest pile.

## The five strategies

Each shrinks the footprint by re-encoding the same information-theoretic bits — nothing is lost.

### Indexes instead of pointers

A pointer doubles on 64-bit CPUs. Keep the objects in an array and reference by index (a **handle**): halves that field and can drop the struct's alignment from 8 to 4, reclaiming padding. Caveat: a bare index is untyped — pass the wrong one and nothing complains. Prefer a language with distinct integer types, or wrap the index. ("Handles are the better pointers", Andreas Weis, has the how.)

### Booleans out of band

A bool is one bit of information but a whole byte plus alignment padding in a struct. Instead of an `alive` flag, keep alive instances in one array and dead ones in another — _which array_ the instance sits in _is_ the flag, for free. Bonus: a loop over the alive array never loads the flag, so there is no branch, no load, and no cache miss spent skipping the dead.

### Struct-of-arrays kills padding

An array of structs pads every element up to its alignment — 7 wasted bytes per 16-byte element. One array per field (**struct of arrays**) lays same-typed fields wall-to-wall with zero padding, same API. It's a one-line data-structure swap; 10k monsters went 160 KB → 91 KB.

### Sparse data out of band

When most instances never use a field (90% of monsters carry nothing), the struct pays for it on every one. Move it out into a hashmap keyed by the instance's index: present when needed, absent when not. 10k monsters with 10% carrying: 366 KB → 198 KB including table overhead.

### Encodings instead of polymorphism

A tagged union or a base+derived tree pays for the fattest variant on every instance. Instead: one tag plus a few repurposed fields; what varies between instances (bee color, human braces) dissolves into the tag. Each distinct arrangement is an **encoding**, and you choose your set of encodings from your actual distribution — half naked humans, half clothed ⇒ 17 bytes vs 32. This is out-of-band applied to the whole struct.

## Layout facts

- Every type has a **natural alignment** and a size. A struct lays fields down one by one, padding each up to that field's alignment, then pads the tail up to the struct's own alignment. Reordering fields (two `u32`s before a `u64`) reclaims padding at the same alignment.
- Same-size-every-variant layouts (a tagged union in one array) are the convenient baseline; the encoding strategy trades that for average footprint.
