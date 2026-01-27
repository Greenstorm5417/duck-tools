/**
 * Semantic token configuration for DuckyScript
 * This defines how tokens are styled in VS Code
 */

export interface SemanticTokenConfig {
    tokenTypes: string[];
    tokenModifiers: string[];
}

/**
 * Semantic token types legend
 * These map to VS Code's built-in semantic token types
 * Custom types can be defined and styled in themes
 */
export const SEMANTIC_TOKEN_TYPES: string[] = [
    'keyword',          // 0 - Control flow, commands
    'function',         // 1 - Function definitions and calls
    'variable',         // 2 - Variable names
    'string',           // 3 - String literals
    'number',           // 4 - Numeric values
    'comment',          // 5 - Comments
    'macro',            // 6 - Preprocessor directives
    'operator',         // 7 - Operators (=, +, -, etc.)
    'modifier',         // 8 - Modifier keys (CTRL, SHIFT, ALT)
    'parameter',        // 9 - Function parameters
    'property',         // 10 - Object properties
    'type',             // 11 - Type names
    'class',            // 12 - Class names
    'namespace',        // 13 - Namespace/module names
];

/**
 * Semantic token modifiers
 * These add additional styling information to tokens
 */
export const SEMANTIC_TOKEN_MODIFIERS: string[] = [
    'declaration',      // 0x01 - Variable/function declaration
    'definition',       // 0x02 - Definition site
    'readonly',         // 0x04 - Read-only variable
    'static',           // 0x08 - Static member
    'deprecated',       // 0x10 - Deprecated symbol
    'abstract',         // 0x20 - Abstract class/method
    'async',            // 0x40 - Async function
    'modification',     // 0x80 - Modifying operation
];

/**
 * Get semantic token configuration for LSP
 */
export function getSemanticTokenConfig(): SemanticTokenConfig {
    return {
        tokenTypes: SEMANTIC_TOKEN_TYPES,
        tokenModifiers: SEMANTIC_TOKEN_MODIFIERS,
    };
}

/**
 * Token type to index mapping
 */
export enum TokenTypeIndex {
    Keyword = 0,
    Function = 1,
    Variable = 2,
    String = 3,
    Number = 4,
    Comment = 5,
    Macro = 6,
    Operator = 7,
    Modifier = 8,
    Parameter = 9,
    Property = 10,
    Type = 11,
    Class = 12,
    Namespace = 13,
}

/**
 * Token modifier bits
 */
export enum TokenModifierBits {
    Declaration = 0x01,
    Definition = 0x02,
    Readonly = 0x04,
    Static = 0x08,
    Deprecated = 0x10,
    Abstract = 0x20,
    Async = 0x40,
    Modification = 0x80,
}
