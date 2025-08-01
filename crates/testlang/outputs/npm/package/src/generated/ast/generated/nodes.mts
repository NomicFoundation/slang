// This file is generated automatically by infrastructure scripts (crates/codegen/runner/src/main.rs:84:22). Please don't edit by hand.

import * as wasm from "../../../../wasm/index.mjs";
import { NonterminalKind, NonterminalNode, TerminalNode } from "../../cst/index.mjs";

//
// Sequences:
//

/**
 * This node represents a `SourceUnit` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnit = (* members: *) SourceUnitMembers;
 * ```
 */
export class SourceUnit {
  private readonly fetch = once(() => {
    const [$members] = wasm.ast.Selectors.sequence(this.cst);

    return {
      members: new SourceUnitMembers($members as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `SourceUnit`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `SourceUnit`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.SourceUnit);
  }

  /**
   * Returns the child node that has the label `members`.
   */
  public get members(): SourceUnitMembers {
    return this.fetch().members;
  }
}

/**
 * This node represents a `Tree` nonterminal, with the following structure:
 *
 * ```ebnf
 * Tree = (* keyword: *) TREE_KEYWORD
 *        (* name: *) IDENTIFIER?
 *        (* node: *) TreeNode
 *        (* semicolon: *) SEMICOLON;
 * ```
 */
export class Tree {
  private readonly fetch = once(() => {
    const [$keyword, $name, $node, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      keyword: $keyword as TerminalNode,
      name: $name === undefined ? undefined : ($name as TerminalNode),
      node: new TreeNode($node as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `Tree`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `Tree`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.Tree);
  }

  /**
   * Returns the child node that has the label `keyword`.
   */
  public get keyword(): TerminalNode {
    return this.fetch().keyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `node`.
   */
  public get node(): TreeNode {
    return this.fetch().node;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `TreeNode` nonterminal, with the following structure:
 *
 * ```ebnf
 * TreeNode = (* open_bracket: *) OPEN_BRACKET
 *            (* members: *) TreeNodeChildren
 *            (* close_bracket: *) CLOSE_BRACKET;
 * ```
 */
export class TreeNode {
  private readonly fetch = once(() => {
    const [$openBracket, $members, $closeBracket] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openBracket: $openBracket as TerminalNode,
      members: new TreeNodeChildren($members as NonterminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `TreeNode`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TreeNode`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TreeNode);
  }

  /**
   * Returns the child node that has the label `open_bracket`.
   */
  public get openBracket(): TerminalNode {
    return this.fetch().openBracket;
  }

  /**
   * Returns the child node that has the label `members`.
   */
  public get members(): TreeNodeChildren {
    return this.fetch().members;
  }

  /**
   * Returns the child node that has the label `close_bracket`.
   */
  public get closeBracket(): TerminalNode {
    return this.fetch().closeBracket;
  }
}

/**
 * This node represents a `AdditionExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * AdditionExpression = (* left_operand: *) Expression
 *                      (* operator: *) PLUS
 *                      (* right_operand: *) Expression;
 * ```
 */
export class AdditionExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `AdditionExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `AdditionExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.AdditionExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `NegationExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Prefix unary operator *)
 * NegationExpression = (* operator: *) BANG
 *                      (* operand: *) Expression;
 * ```
 */
export class NegationExpression {
  private readonly fetch = once(() => {
    const [$operator, $operand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operator: $operator as TerminalNode,
      operand: new Expression($operand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `NegationExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `NegationExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.NegationExpression);
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): Expression {
    return this.fetch().operand;
  }
}

/**
 * This node represents a `MemberAccessExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * MemberAccessExpression = (* operand: *) Expression
 *                          (* period: *) PERIOD
 *                          (* member: *) IDENTIFIER;
 * ```
 */
export class MemberAccessExpression {
  private readonly fetch = once(() => {
    const [$operand, $period, $member] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operand: new Expression($operand as NonterminalNode),
      period: $period as TerminalNode,
      member: $member as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `MemberAccessExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `MemberAccessExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.MemberAccessExpression);
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): Expression {
    return this.fetch().operand;
  }

  /**
   * Returns the child node that has the label `period`.
   */
  public get period(): TerminalNode {
    return this.fetch().period;
  }

  /**
   * Returns the child node that has the label `member`.
   */
  public get member(): TerminalNode {
    return this.fetch().member;
  }
}

//
// Choices:
//

/**
 * This node represents a `SourceUnitMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnitMember = (* variant: *) Tree
 *                  | (* variant: *) Expression
 *                  | (* variant: *) SeparatedIdentifiers
 *                  | (* variant: *) Literal;
 * ```
 */
export class SourceUnitMember {
  private readonly fetch: () => Tree | Expression | SeparatedIdentifiers | Literal = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.Tree:
        return new Tree(variant as NonterminalNode);
      case NonterminalKind.Expression:
        return new Expression(variant as NonterminalNode);
      case NonterminalKind.SeparatedIdentifiers:
        return new SeparatedIdentifiers(variant as NonterminalNode);
      case NonterminalKind.Literal:
        return new Literal(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `SourceUnitMember`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `SourceUnitMember`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.SourceUnitMember);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): Tree | Expression | SeparatedIdentifiers | Literal {
    return this.fetch();
  }
}

/**
 * This node represents a `TreeNodeChild` nonterminal, with the following structure:
 *
 * ```ebnf
 * TreeNodeChild = (* variant: *) TreeNode
 *               | (* variant: *) DELIMITED_IDENTIFIER;
 * ```
 */
export class TreeNodeChild {
  private readonly fetch: () => TreeNode | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.TreeNode:
        return new TreeNode(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `TreeNodeChild`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TreeNodeChild`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TreeNodeChild);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TreeNode | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `Expression` nonterminal, with the following structure:
 *
 * ```ebnf
 * Expression = (* variant: *) AdditionExpression
 *            | (* variant: *) NegationExpression
 *            | (* variant: *) MemberAccessExpression
 *            | (* variant: *) STRING_LITERAL
 *            | (* variant: *) IDENTIFIER;
 * ```
 */
export class Expression {
  private readonly fetch: () => AdditionExpression | NegationExpression | MemberAccessExpression | TerminalNode = once(
    () => {
      const variant = wasm.ast.Selectors.choice(this.cst);

      if (variant.isTerminalNode()) {
        return variant;
      }

      switch (variant.kind) {
        case NonterminalKind.AdditionExpression:
          return new AdditionExpression(variant as NonterminalNode);
        case NonterminalKind.NegationExpression:
          return new NegationExpression(variant as NonterminalNode);
        case NonterminalKind.MemberAccessExpression:
          return new MemberAccessExpression(variant as NonterminalNode);

        default:
          throw new Error(`Unexpected variant: '${variant.kind}'.`);
      }
    },
  );

  /**
   * Constructs a new AST node of type `Expression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `Expression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.Expression);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): AdditionExpression | NegationExpression | MemberAccessExpression | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `Literal` nonterminal, with the following structure:
 *
 * ```ebnf
 * Literal = (* variant: *) STRING_LITERAL;
 * ```
 */
export class Literal {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `Literal`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `Literal`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.Literal);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

//
// Repeated:
//

/**
 * This node represents a `SourceUnitMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnitMembers = (* item: *) SourceUnitMember+;
 * ```
 */
export class SourceUnitMembers {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new SourceUnitMember(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `SourceUnitMembers`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `SourceUnitMembers`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.SourceUnitMembers);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly SourceUnitMember[] {
    return this.fetch();
  }
}

/**
 * This node represents a `TreeNodeChildren` nonterminal, with the following structure:
 *
 * ```ebnf
 * TreeNodeChildren = (* item: *) TreeNodeChild+;
 * ```
 */
export class TreeNodeChildren {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new TreeNodeChild(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `TreeNodeChildren`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TreeNodeChildren`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TreeNodeChildren);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly TreeNodeChild[] {
    return this.fetch();
  }
}

//
// Separated:
//

/**
 * This node represents a `SeparatedIdentifiers` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 1.0.0 *)
 * SeparatedIdentifiers = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
 * ```
 */
export class SeparatedIdentifiers {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  /**
   * Constructs a new AST node of type `SeparatedIdentifiers`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `SeparatedIdentifiers`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.SeparatedIdentifiers);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

function once<T>(factory: () => T): () => T {
  let value: T | undefined;
  return () => {
    if (value === undefined) {
      value = factory();
    }
    return value;
  };
}

function assertKind(actual: NonterminalKind, expected: NonterminalKind): void {
  if (actual !== expected) {
    throw new Error(
      `AST node '${expected}' can only be initialized with a CST node of the same kind. Received '${actual}' instead.`,
    );
  }
}
