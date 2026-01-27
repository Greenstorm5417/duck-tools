"use strict";
/**
 * Semantic token configuration for DuckyScript
 * This defines how tokens are styled in VS Code
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.TokenModifierBits = exports.TokenTypeIndex = exports.SEMANTIC_TOKEN_MODIFIERS = exports.SEMANTIC_TOKEN_TYPES = void 0;
exports.getSemanticTokenConfig = getSemanticTokenConfig;
/**
 * Semantic token types legend
 * These map to VS Code's built-in semantic token types
 * Custom types can be defined and styled in themes
 */
exports.SEMANTIC_TOKEN_TYPES = [
    'keyword', // 0 - Control flow, commands
    'function', // 1 - Function definitions and calls
    'variable', // 2 - Variable names
    'string', // 3 - String literals
    'number', // 4 - Numeric values
    'comment', // 5 - Comments
    'macro', // 6 - Preprocessor directives
    'operator', // 7 - Operators (=, +, -, etc.)
    'modifier', // 8 - Modifier keys (CTRL, SHIFT, ALT)
    'parameter', // 9 - Function parameters
    'property', // 10 - Object properties
    'type', // 11 - Type names
    'class', // 12 - Class names
    'namespace', // 13 - Namespace/module names
];
/**
 * Semantic token modifiers
 * These add additional styling information to tokens
 */
exports.SEMANTIC_TOKEN_MODIFIERS = [
    'declaration', // 0x01 - Variable/function declaration
    'definition', // 0x02 - Definition site
    'readonly', // 0x04 - Read-only variable
    'static', // 0x08 - Static member
    'deprecated', // 0x10 - Deprecated symbol
    'abstract', // 0x20 - Abstract class/method
    'async', // 0x40 - Async function
    'modification', // 0x80 - Modifying operation
];
/**
 * Get semantic token configuration for LSP
 */
function getSemanticTokenConfig() {
    return {
        tokenTypes: exports.SEMANTIC_TOKEN_TYPES,
        tokenModifiers: exports.SEMANTIC_TOKEN_MODIFIERS,
    };
}
/**
 * Token type to index mapping
 */
var TokenTypeIndex;
(function (TokenTypeIndex) {
    TokenTypeIndex[TokenTypeIndex["Keyword"] = 0] = "Keyword";
    TokenTypeIndex[TokenTypeIndex["Function"] = 1] = "Function";
    TokenTypeIndex[TokenTypeIndex["Variable"] = 2] = "Variable";
    TokenTypeIndex[TokenTypeIndex["String"] = 3] = "String";
    TokenTypeIndex[TokenTypeIndex["Number"] = 4] = "Number";
    TokenTypeIndex[TokenTypeIndex["Comment"] = 5] = "Comment";
    TokenTypeIndex[TokenTypeIndex["Macro"] = 6] = "Macro";
    TokenTypeIndex[TokenTypeIndex["Operator"] = 7] = "Operator";
    TokenTypeIndex[TokenTypeIndex["Modifier"] = 8] = "Modifier";
    TokenTypeIndex[TokenTypeIndex["Parameter"] = 9] = "Parameter";
    TokenTypeIndex[TokenTypeIndex["Property"] = 10] = "Property";
    TokenTypeIndex[TokenTypeIndex["Type"] = 11] = "Type";
    TokenTypeIndex[TokenTypeIndex["Class"] = 12] = "Class";
    TokenTypeIndex[TokenTypeIndex["Namespace"] = 13] = "Namespace";
})(TokenTypeIndex || (exports.TokenTypeIndex = TokenTypeIndex = {}));
/**
 * Token modifier bits
 */
var TokenModifierBits;
(function (TokenModifierBits) {
    TokenModifierBits[TokenModifierBits["Declaration"] = 1] = "Declaration";
    TokenModifierBits[TokenModifierBits["Definition"] = 2] = "Definition";
    TokenModifierBits[TokenModifierBits["Readonly"] = 4] = "Readonly";
    TokenModifierBits[TokenModifierBits["Static"] = 8] = "Static";
    TokenModifierBits[TokenModifierBits["Deprecated"] = 16] = "Deprecated";
    TokenModifierBits[TokenModifierBits["Abstract"] = 32] = "Abstract";
    TokenModifierBits[TokenModifierBits["Async"] = 64] = "Async";
    TokenModifierBits[TokenModifierBits["Modification"] = 128] = "Modification";
})(TokenModifierBits || (exports.TokenModifierBits = TokenModifierBits = {}));
//# sourceMappingURL=semanticTokens.js.map