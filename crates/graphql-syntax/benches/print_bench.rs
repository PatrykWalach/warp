/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fmt::Write;

use common::SourceLocationKey;
use graphql_syntax::parse_schema_document;
use graphql_syntax::SchemaDocument;

const SCHEMA: &str = include_str!("../tests/print/fixtures/schema.graphql");

fn main() {
    // Run registered benchmarks.
    divan::main();
}

fn schema_document() -> SchemaDocument {
    parse_schema_document(SCHEMA, SourceLocationKey::standalone("schema.graphql"))
        .expect("Failed to parse the schema fixture")
}

/// Only the printing side of the round trip: the document is parsed in the
/// (untimed) input setup.
#[divan::bench]
fn print_schema(bencher: divan::Bencher) {
    bencher
        .with_inputs(schema_document)
        .bench_values(|document| {
            let mut output = String::new();
            for definition in &document.definitions {
                write!(output, "{definition}").expect("Failed to print definition");
            }
            output
        });
}

/// Parse then print, the operation a formatter performs.
#[divan::bench]
fn parse_and_print_schema(bencher: divan::Bencher) {
    bencher.with_inputs(|| SCHEMA).bench_values(|input| {
        let document =
            parse_schema_document(input, SourceLocationKey::standalone("schema.graphql"))
                .expect("Failed to parse the schema fixture");
        let mut output = String::new();
        for definition in &document.definitions {
            write!(output, "{definition}").expect("Failed to print definition");
        }
        output
    });
}
