/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::SourceLocationKey;
use graphql_syntax::parse_schema_document;
use graphql_syntax::parse_type;

const SCHEMA_KITCHEN_SINK: &str =
    include_str!("../tests/parse_schema_document/fixtures/schema_kitchen_sink.graphql");
const TYPE_DEFINITION: &str =
    include_str!("../tests/parse_schema_document/fixtures/type_definition.graphql");

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn schema_kitchen_sink(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| SCHEMA_KITCHEN_SINK)
        .bench_values(|input| {
            parse_schema_document(
                input,
                SourceLocationKey::standalone("schema_kitchen_sink.graphql"),
            )
        });
}

/// Schemas are the largest documents the parser has to handle, so also measure
/// a scaled up version of the kitchen sink schema.
#[divan::bench]
fn schema_kitchen_sink_x16(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| SCHEMA_KITCHEN_SINK.repeat(16))
        .bench_values(|input| {
            parse_schema_document(
                &input,
                SourceLocationKey::standalone("schema_kitchen_sink.graphql"),
            )
        });
}

#[divan::bench]
fn type_definition(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| TYPE_DEFINITION)
        .bench_values(|input| {
            parse_schema_document(
                input,
                SourceLocationKey::standalone("type_definition.graphql"),
            )
        });
}

/// Parsing a single type annotation is a very hot, very small operation.
#[divan::bench]
fn type_annotation(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| "[[MyEnum!]!]!")
        .bench_values(|input| parse_type(input, SourceLocationKey::standalone("type.graphql"), 0));
}
