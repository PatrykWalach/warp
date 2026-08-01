/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::SourceLocationKey;
use graphql_syntax::parse_executable;
use graphql_syntax::parse_executable_with_error_recovery;

const KITCHEN_SINK: &str =
    include_str!("../tests/parse_executable_document/fixtures/kitchen-sink.graphql");
const MULTIPLE_PARSE_ERRORS: &str = include_str!(
    "../tests/parse_executable_document/fixtures/multiple_parse_errors.invalid.graphql"
);
const BLOCK_STRING: &str =
    include_str!("../tests/parse_executable_document/fixtures/block_string.graphql");

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn kitchen_sink(bencher: divan::Bencher) {
    bencher.with_inputs(|| KITCHEN_SINK).bench_values(|input| {
        parse_executable(input, SourceLocationKey::standalone("kitchen-sink.graphql"))
    });
}

/// A document made of 32 copies of the kitchen sink, to measure the parser on
/// an input size closer to a real project's set of queries.
#[divan::bench]
fn kitchen_sink_x32(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| KITCHEN_SINK.repeat(32))
        .bench_values(|input| {
            parse_executable(
                &input,
                SourceLocationKey::standalone("kitchen-sink.graphql"),
            )
        });
}

#[divan::bench]
fn block_string(bencher: divan::Bencher) {
    bencher.with_inputs(|| BLOCK_STRING).bench_values(|input| {
        parse_executable(input, SourceLocationKey::standalone("block_string.graphql"))
    });
}

/// The error recovery path is what the LSP hits on every keystroke, so it is
/// worth tracking on its own.
#[divan::bench]
fn multiple_parse_errors_with_error_recovery(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| MULTIPLE_PARSE_ERRORS)
        .bench_values(|input| {
            parse_executable_with_error_recovery(
                input,
                SourceLocationKey::standalone("multiple_parse_errors.invalid.graphql"),
            )
        });
}
