<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #TypeName }
TypeName = ArrayTypeName (* TypeName «ArrayTypeNameOperator» *) (* Unary Operator, Postfix *)
         | FunctionType
         | MappingType
         | «ElementaryType»
         | IdentifierPath;
```

```{ .ebnf .slang-ebnf #ArrayTypeName }
ArrayTypeName = TypeName «ArrayTypeNameOperator» (* Unary Operator, Postfix *);
```
