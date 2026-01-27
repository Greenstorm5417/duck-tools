import * as fs from 'fs';
import * as path from 'path';

/**
 * Extension definition from extensions.json
 */
export interface DuckyExtension {
    name: string;
    extension_name: string;
    extension_version: string;
    code: string;
}

/**
 * Extension completion provider
 * Loads extensions from extensions.json and provides auto-completion
 */
export class ExtensionCompletionProvider {
    private extensions: DuckyExtension[] = [];
    
    constructor(extensionPath: string) {
        this.loadExtensions(extensionPath);
    }
    
    /**
     * Load extensions from extensions.json
     */
    private loadExtensions(extensionPath: string): void {
        const extensionsFile = path.join(extensionPath, 'src', 'extensions.json');
        
        try {
            if (fs.existsSync(extensionsFile)) {
                const content = fs.readFileSync(extensionsFile, 'utf-8');
                this.extensions = JSON.parse(content);
                console.log(`Loaded ${this.extensions.length} DuckyScript extensions`);
            } else {
                console.warn(`Extensions file not found: ${extensionsFile}`);
            }
        } catch (error) {
            console.error('Failed to load extensions.json:', error);
        }
    }
    
    /**
     * Get all extension names for auto-completion
     */
    public getExtensionNames(): string[] {
        return this.extensions.map(ext => {
            // Extract just the extension name without "EXTENSION " prefix
            return ext.name.replace(/^EXTENSION\s+/, '');
        });
    }
    
    /**
     * Get extension by name
     */
    public getExtension(name: string): DuckyExtension | undefined {
        return this.extensions.find(ext => 
            ext.name === name || 
            ext.name === `EXTENSION ${name}` ||
            ext.extension_name === name
        );
    }
    
    /**
     * Get extension code for insertion
     */
    public getExtensionCode(name: string): string | undefined {
        const ext = this.getExtension(name);
        return ext?.code;
    }
    
    /**
     * Get all extensions for completion items
     */
    public getAllExtensions(): DuckyExtension[] {
        return this.extensions;
    }
}
