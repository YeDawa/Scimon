# Conditionals

Evaluate different blocks of Monset code depending on variables using `if` and `else` conditional structures.

## Syntax

```scimon
if operand1 == operand2 {
    // block executed if condition is true
} else {
    // block executed if condition is false (optional)
}
```

- **Operators**: Supports both equality (`==`) and inequality (`!=`) comparisons.
- **Operands**: Can be double-quoted strings (e.g. `"prod"`), single-quoted strings (e.g. `'dev'`), or bare words (e.g. `prod`, `dev`, numbers). They are resolved after variables, allowing comparisons with variable values (e.g. `${ENV}`).

## Example Usage

### Environment-based Port Configuration

```scimon
@var ENV "prod"

if ${ENV} == "prod" {
    server "80"
} else {
    server "8080"
}
```

If the `${ENV}` variable is `"prod"`, the port `80` is configured; otherwise, it configures port `8080`.

### Conditional Downloads

```scimon
@var GET_EXTRA_PAPERS "yes"

downloads {
    https://arxiv.org/pdf/2203.08877 as "primary.pdf"
    
    if ${GET_EXTRA_PAPERS} == "yes" {
        https://arxiv.org/pdf/2405.01513 as "extra.pdf"
    }
}
```

## Notes

- `if` blocks can nest inside other blocks like `downloads { ... }` or `group "name" { ... }`.
- Conditionals are expanded in a preprocessing pass *after* variable interpolation has run, so the body can evaluate variables directly.
- The `else` block is optional.
