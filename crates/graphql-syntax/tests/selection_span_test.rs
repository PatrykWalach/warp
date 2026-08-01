/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::SourceLocationKey;
use graphql_syntax::parse_executable;
use graphql_syntax::ExecutableDefinition;
use graphql_syntax::Selection;

const EXECUTABLE: &str = r#"
query EmptyQuery($id: ID!) {
  node(id: $id) {
    id @skip(if: false)
    ... on User {
      ...E1
    }
  }
}
fragment E1 on User {
  name
  friends(first: 10) {
    name
  }
}
"#;

#[test]
fn selection_spans_slice_the_source() {
    let doc = parse_executable(EXECUTABLE, SourceLocationKey::standalone("exec.graphql"))
        .expect("fixture must parse");

    let mut texts = Vec::new();
    for definition in &doc.definitions {
        let selections = match definition {
            ExecutableDefinition::Operation(node) => &node.selections,
            ExecutableDefinition::Fragment(node) => &node.selections,
        };
        walk(selections, &mut texts);
    }

    let expected = vec![
        "node(id: $id) {\n    id @skip(if: false)\n    ... on User {\n      ...E1\n    }\n  }",
        "id @skip(if: false)",
        "... on User {\n      ...E1\n    }",
        "...E1",
        "name",
        "friends(first: 10) {\n    name\n  }",
        "name",
    ];

    assert_eq!(texts, expected);
}

fn walk<'a>(list: &'a graphql_syntax::List<Selection>, out: &mut Vec<String>) {
    for selection in &list.items {
        let (start, end) = selection.span().as_usize();
        out.push(EXECUTABLE[start..end].trim().to_string());
        match selection {
            Selection::LinkedField(node) => walk(&node.selections, out),
            Selection::InlineFragment(node) => walk(&node.selections, out),
            Selection::ScalarField(_) | Selection::FragmentSpread(_) => {}
        }
    }
}
