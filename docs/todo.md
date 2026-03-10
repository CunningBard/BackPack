# BackPack TODO

## Loops
- [x] `for x in iterable` loop — grammar, AST, parser, translator

## Data Types
- [x] List / array / slice type — grammar, AST, parser, translator

## Assignment
- [x] `+=` reassignment operator (desugared to addition assignment in parser)

## Structures
- [ ] `struct` declaration — grammar, AST (`Construct::Struct`), parser, translator (Go struct codegen)
- [ ] Struct literal expressions — `{ name: "jack", age: 5 }`
- [ ] Struct as typed arg — `GET say_hello(person: Person)` // basically the json part of the payload
- [ ] `Struct(String)` type variant in AST (currently commented out)

## Expressions
- [x] `&&` and `||` logical operators — grammar, AST, parser, translator
- [x] Unary minus on variables — `-x`

## Specifiers
- [x] `#HEADER name` — grammar, AST (`Handler`), parser, translator (extract from `r.Header.Get(...)`)
- [x] `#HEADER EXPECT key: value` — grammar, AST, parser, translator (check header or reject request)

## Imports
- [ ] `import <path>` — grammar, AST, parser, translator
- [ ] `from <path> import <identifier>` — grammar, AST, parser, translator

## Status Codes
- [ ] `Status.OK`, `Status.NOT_FOUND`, etc. — map to `http.StatusOK`, `http.StatusNotFound`, etc. in translator

## Type Inference
- [ ] `let name = "jack"` without type annotation — currently panics; needs type inference pass

## Comments
- [ ] Multi-line `/* */` comments spanning real newlines — current grammar may misbehave
