/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @generated SignedSource<<c19133fff638ad163c66db8cdfbd3294>>
 */

use common::SourceLocationKey;
use graphql_syntax::parse_document;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn invalid_definition_invalid(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            include_str!("../tests/parse_document/fixtures/invalid_definition.invalid.graphql")
        })
        .bench_values(|input| {
            parse_document(
                input,
                SourceLocationKey::standalone("invalid_definition.invalid.graphql"),
            )
        });
}

#[divan::bench]
fn mixed(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| include_str!("../tests/parse_document/fixtures/mixed.graphql"))
        .bench_values(|input| {
            parse_document(input, SourceLocationKey::standalone("mixed.graphql"))
        });
}
