/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use signedsource::is_signed;
use signedsource::is_valid_signature;
use signedsource::sign_file;
use signedsource::SIGNING_TOKEN;

/// A generated artifact big enough to be representative: the compiler signs one
/// file per GraphQL operation.
fn unsigned_file(definitions: usize) -> String {
    let mut file = format!("/**\n * {SIGNING_TOKEN}\n */\n\n");
    for i in 0..definitions {
        file.push_str(&format!(
            "export const Query_{i} = {{\n  kind: \"Request\",\n  name: \"Query_{i}\",\n  selections: [\"id\", \"name\", \"createdAt\"],\n}};\n\n"
        ));
    }
    file
}

/// The signature regex is lazily compiled on first use. Compiling it in the
/// (untimed) input setup keeps that one-off cost out of the measurements.
fn warm_up_regex() {
    is_signed(SIGNING_TOKEN);
}

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn sign_small_file(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| unsigned_file(8))
        .bench_values(|file| sign_file(&file));
}

#[divan::bench]
fn sign_large_file(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| unsigned_file(512))
        .bench_values(|file| sign_file(&file));
}

/// Signature verification is what the compiler runs on every already generated
/// artifact to decide whether it can be overwritten.
#[divan::bench]
fn verify_large_file(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            warm_up_regex();
            sign_file(&unsigned_file(512))
        })
        .bench_values(|file| is_valid_signature(&file));
}

/// The regex-only path, without hashing.
#[divan::bench]
fn is_signed_large_file(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            warm_up_regex();
            sign_file(&unsigned_file(512))
        })
        .bench_values(|file| is_signed(&file));
}
