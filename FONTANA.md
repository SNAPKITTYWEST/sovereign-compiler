# FONTANA.md — Fontana DSL Reference

## Grammar

```
program     := declaration*
declaration := 'decl' IDENT '{' statement* '}'
             | 'let' IDENT '=' expression ';'
             | expression ';'

statement   := declaration
             | 'return' expression ';'
             | expression ';'

expression  := literal
             | IDENT
             | expression binary_op expression
             | IDENT '(' argument_list ')'
             | 'if' '(' expression ')' '{' statement* '}' ('else' '{' statement* '}')?
             | 'verify' '(' expression ')'
             | 'seal' '(' expression ')'

literal     := INTEGER | FLOAT | STRING | BOOL | 'null'

binary_op   := '+' | '-' | '*' | '/' | '==' | '!=' | '<' | '>' | '<=' | '>='
```

## Example

```fontana
decl counter {
  let x = 42;
  verify(x > 0);
  seal(x);
  return x;
}
```

## Compilation

1. Parse to AST
2. Validate admissibility
3. Check stratum boundaries
4. Compute contractivity
5. Apply governance policies
6. Verify with Lean
7. Execute in Rust
8. Generate WORM witness
9. Persist to Archivum
10. Record in Observatory
