"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.ExtensionCompletionProvider = void 0;
const fs = __importStar(require("fs"));
const path = __importStar(require("path"));
/**
 * Extension completion provider
 * Loads extensions from extensions.json and provides auto-completion
 */
class ExtensionCompletionProvider {
    constructor(extensionPath) {
        this.extensions = [];
        this.loadExtensions(extensionPath);
    }
    /**
     * Load extensions from extensions.json
     */
    loadExtensions(extensionPath) {
        const extensionsFile = path.join(extensionPath, 'src', 'extensions.json');
        try {
            if (fs.existsSync(extensionsFile)) {
                const content = fs.readFileSync(extensionsFile, 'utf-8');
                this.extensions = JSON.parse(content);
                console.log(`Loaded ${this.extensions.length} DuckyScript extensions`);
            }
            else {
                console.warn(`Extensions file not found: ${extensionsFile}`);
            }
        }
        catch (error) {
            console.error('Failed to load extensions.json:', error);
        }
    }
    /**
     * Get all extension names for auto-completion
     */
    getExtensionNames() {
        return this.extensions.map(ext => {
            // Extract just the extension name without "EXTENSION " prefix
            return ext.name.replace(/^EXTENSION\s+/, '');
        });
    }
    /**
     * Get extension by name
     */
    getExtension(name) {
        return this.extensions.find(ext => ext.name === name ||
            ext.name === `EXTENSION ${name}` ||
            ext.extension_name === name);
    }
    /**
     * Get extension code for insertion
     */
    getExtensionCode(name) {
        const ext = this.getExtension(name);
        return ext?.code;
    }
    /**
     * Get all extensions for completion items
     */
    getAllExtensions() {
        return this.extensions;
    }
}
exports.ExtensionCompletionProvider = ExtensionCompletionProvider;
//# sourceMappingURL=extensionCompletions.js.map