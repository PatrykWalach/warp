/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use intern::string;
use intern::string::StringId;
use intern::string_key::Intern;
use intern::string_key::StringKey;
use intern::string_key::StringKeyMap;
use intern::Lookup;

/// Roughly the number of distinct identifiers a medium sized GraphQL project
/// interns while parsing.
const CORPUS_SIZE: usize = 1_024;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

fn corpus() -> Vec<String> {
    (0..CORPUS_SIZE)
        .map(|i| format!("SomeReasonablyLongFieldOrTypeName_{i}"))
        .collect()
}

/// The dominant path in a parser: the string is already interned, so this
/// measures the sharded hash lookup.
#[divan::bench]
fn intern_existing(bencher: divan::Bencher) {
    let words = corpus();
    for word in &words {
        string::intern(word.as_str());
    }
    bencher.bench(|| {
        for word in &words {
            divan::black_box(string::intern(divan::black_box(word.as_str())));
        }
    });
}

/// The cold path: every string is new, so it has to be added to the arena.
#[divan::bench]
fn intern_new(bencher: divan::Bencher) {
    let mut generation = 0usize;
    bencher
        .with_inputs(|| {
            generation += 1;
            (0..CORPUS_SIZE)
                .map(|i| format!("Generation{generation}_Name{i}"))
                .collect::<Vec<String>>()
        })
        .bench_local_values(|words| {
            for word in &words {
                divan::black_box(string::intern(word.as_str()));
            }
        });
}

/// Resolving an id back to its `&'static str`.
#[divan::bench]
fn lookup(bencher: divan::Bencher) {
    let ids: Vec<StringId> = corpus()
        .iter()
        .map(|word| string::intern(word.as_str()))
        .collect();
    bencher.bench(|| {
        for id in &ids {
            divan::black_box(id.as_str());
        }
    });
}

/// `StringKey` is hashed with `IdHasher`, this is what every symbol table
/// lookup in the compiler costs.
#[divan::bench]
fn string_key_map(bencher: divan::Bencher) {
    let keys: Vec<StringKey> = corpus().iter().map(|word| word.as_str().intern()).collect();
    bencher.bench(|| {
        let mut map: StringKeyMap<usize> = StringKeyMap::default();
        for (index, key) in keys.iter().enumerate() {
            map.insert(*key, index);
        }
        for key in &keys {
            divan::black_box(map.get(key));
        }
        map
    });
}

/// `Lookup::lookup` through the `StringKey` wrapper.
#[divan::bench]
fn string_key_lookup(bencher: divan::Bencher) {
    let keys: Vec<StringKey> = corpus().iter().map(|word| word.as_str().intern()).collect();
    bencher.bench(|| {
        for key in &keys {
            divan::black_box(key.lookup());
        }
    });
}
