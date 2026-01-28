import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';
import { workspace, ExtensionContext, window, commands, Uri, languages, CompletionItem, CompletionItemKind, SnippetString } from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable
} from 'vscode-languageclient/node';
import { ExtensionCompletionProvider } from './extensionCompletions';

let client: LanguageClient;
let extensionProvider: ExtensionCompletionProvider;

function findDuckCli(): string | null {
    const config = workspace.getConfiguration('duckyscript');
    const customPath = config.get<string>('cli.path', '');

    if (customPath) {
        if (fs.existsSync(customPath)) {
            return customPath;
        }
        window.showWarningMessage(`Custom CLI path not found: ${customPath}`);
        return null;
    }

    const isWindows = process.platform === 'win32';
    const exeName = isWindows ? 'duck.exe' : 'duck';

    const pathEnv = process.env.PATH || '';
    const pathDirs = pathEnv.split(isWindows ? ';' : ':');

    for (const dir of pathDirs) {
        const candidate = path.join(dir, exeName);
        if (fs.existsSync(candidate)) {
            return candidate;
        }
    }

    const homeDir = os.homedir();
    const standardLocations = [
        path.join(homeDir, '.duck', 'bin', exeName),
        path.join(homeDir, '.cargo', 'bin', exeName),
    ];

    if (isWindows) {
        standardLocations.push(
            path.join('C:', 'Program Files', 'ducky-cli', 'bin', exeName),
            path.join(process.env.LOCALAPPDATA || '', 'Programs', 'ducky-cli', exeName)
        );
    } else {
        standardLocations.push(
            path.join('/usr', 'local', 'bin', exeName),
            path.join('/usr', 'bin', exeName)
        );
    }

    for (const location of standardLocations) {
        if (fs.existsSync(location)) {
            return location;
        }
    }

    return null;
}

function findLspServer(): string | null {
    const config = workspace.getConfiguration('duckyscript');
    const customPath = config.get<string>('lsp.path', '');
    
    // If custom path is specified, use it
    if (customPath) {
        if (fs.existsSync(customPath)) {
            return customPath;
        }
        window.showWarningMessage(`Custom LSP path not found: ${customPath}`);
        return null;
    }
    
    const isWindows = process.platform === 'win32';
    const exeName = isWindows ? 'ducky-lsp.exe' : 'ducky-lsp';
    
    // Check PATH environment variable
    const pathEnv = process.env.PATH || '';
    const pathDirs = pathEnv.split(isWindows ? ';' : ':');
    
    for (const dir of pathDirs) {
        const candidate = path.join(dir, exeName);
        if (fs.existsSync(candidate)) {
            return candidate;
        }
    }
    
    // Check standard install locations
    const homeDir = os.homedir();
    const standardLocations = [
        path.join(homeDir, '.duck', 'bin', exeName),
        path.join(homeDir, '.cargo', 'bin', exeName),
    ];
    
    if (isWindows) {
        standardLocations.push(
            path.join('C:', 'Program Files', 'ducky-lsp', 'bin', exeName),
            path.join(process.env.LOCALAPPDATA || '', 'Programs', 'ducky-lsp', exeName)
        );
    } else {
        standardLocations.push(
            path.join('/usr', 'local', 'bin', exeName),
            path.join('/usr', 'bin', exeName)
        );
    }
    
    for (const location of standardLocations) {
        if (fs.existsSync(location)) {
            return location;
        }
    }
    
    return null;
}

export function activate(context: ExtensionContext) {
    // Get configuration
    const config = workspace.getConfiguration('duckyscript');
    
    // Initialize extension completion provider
    extensionProvider = new ExtensionCompletionProvider(context.extensionPath);
    
    // Register extension completions
    const extensionCompletions = languages.registerCompletionItemProvider(
        { scheme: 'file', language: 'duckyscript' },
        {
            async provideCompletionItems(document, position) {
                const linePrefix = document.lineAt(position).text.substr(0, position.character);
                
                // Trigger on "EXTENSION " prefix
                if (linePrefix.match(/\bEXTENSION\s+\w*$/)) {
                    const items: CompletionItem[] = [];
                    
                    for (const ext of extensionProvider.getAllExtensions()) {
                        const extName = ext.name.replace(/^EXTENSION\s+/, '');
                        const item = new CompletionItem(extName, CompletionItemKind.Module);
                        item.detail = ext.extension_version;
                        item.documentation = `Insert ${extName} extension`;
                        
                        // Insert full extension code
                        const code = extensionProvider.getExtensionCode(extName);
                        if (code) {
                            // Escape $ characters for SnippetString ($ is special syntax for placeholders)
                            const escapedCode = code.replace(/\$/g, '\\$');
                            item.insertText = new SnippetString(escapedCode);
                            
                            // Add command to fold the extension after insertion (if enabled)
                            const autoFold = workspace.getConfiguration('duckyscript').get('extension.autoFold', true);
                            if (autoFold) {
                                item.command = {
                                    command: 'editor.fold',
                                    title: 'Fold Extension',
                                    arguments: [{
                                        levels: 1,
                                        direction: 'down',
                                        selectionLines: [position.line]
                                    }]
                                };
                            }
                        }
                        
                        items.push(item);
                    }
                    
                    return items;
                }
                
                return undefined;
            }
        },
        ' ' // Trigger on space after EXTENSION
    );
    
    context.subscriptions.push(extensionCompletions);
    
    // Find LSP server binary
    const serverPath = findLspServer();
    
    if (!serverPath) {
        window.showErrorMessage(
            'DuckyScript LSP server not found. Please install it or configure the path in settings.',
            'Install Instructions',
            'Configure Path'
        ).then(selection => {
            if (selection === 'Install Instructions') {
                commands.executeCommand('vscode.open', Uri.parse('https://github.com/Greenstorm5417/duck-tools#installation'));
            } else if (selection === 'Configure Path') {
                commands.executeCommand('workbench.action.openSettings', 'duckyscript.lsp.path');
            }
        });
        return;
    }

    const serverExecutable: Executable = {
        command: serverPath,
        args: []
    };

    const serverOptions: ServerOptions = {
        run: serverExecutable,
        debug: serverExecutable
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'duckyscript' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/*.txt'),
            configurationSection: 'duckyscript'
        },
        // Middleware to control LSP features based on configuration
        middleware: {
            provideHover: config.get('lsp.hover.enable', true) ? undefined : () => null,
            provideCompletionItem: config.get('lsp.completion.enable', true) ? undefined : () => null,
            provideDefinition: config.get('lsp.definition.enable', true) ? undefined : () => null,
            provideReferences: config.get('lsp.references.enable', true) ? undefined : () => null,
            provideDocumentSymbols: config.get('lsp.documentSymbol.enable', true) ? undefined : () => null,
            provideDocumentHighlights: config.get('lsp.documentHighlight.enable', true) ? undefined : () => null,
            provideCodeLenses: config.get('lsp.codeLens.enable', true) ? undefined : () => null,
            provideSignatureHelp: config.get('lsp.signatureHelp.enable', true) ? undefined : () => null,
            provideFoldingRanges: config.get('lsp.foldingRange.enable', true) ? undefined : () => null,
            provideCodeActions: config.get('lsp.codeAction.enable', true) ? undefined : () => null,
            provideDocumentFormattingEdits: config.get('lsp.formatting.enable', true) ? undefined : () => null,
            provideDocumentRangeFormattingEdits: config.get('lsp.rangeFormatting.enable', true) ? undefined : () => null,
            provideOnTypeFormattingEdits: config.get('lsp.onTypeFormatting.enable', true) ? undefined : () => null,
            provideRenameEdits: config.get('lsp.rename.enable', true) ? undefined : () => null,
            provideSelectionRanges: config.get('lsp.selectionRange.enable', true) ? undefined : () => null,
            provideLinkedEditingRange: config.get('lsp.linkedEditing.enable', true) ? undefined : () => null,
            prepareCallHierarchy: config.get('lsp.callHierarchy.enable', true) ? undefined : () => null,
            provideInlayHints: config.get('lsp.inlayHint.enable', true) ? undefined : () => null
        }
    };

    client = new LanguageClient(
        'duckyscriptLsp',
        'DuckyScript Language Server',
        serverOptions,
        clientOptions
    );

    // Only start LSP if enabled
    if (config.get('lsp.enable', true)) {
        client.start().catch((error) => {
            window.showErrorMessage(`Failed to start LSP: ${error.message}`);
        });
    }
    
    // Watch for configuration changes
    context.subscriptions.push(
        workspace.onDidChangeConfiguration(e => {
            if (e.affectsConfiguration('duckyscript.lsp.enable')) {
                const enabled = workspace.getConfiguration('duckyscript').get('lsp.enable', true);
                if (enabled && !client.isRunning()) {
                    client.start().catch((error) => {
                        window.showErrorMessage(`Failed to start LSP: ${error.message}`);
                    });
                } else if (!enabled && client.isRunning()) {
                    client.stop();
                }
            }
            
            // Notify about feature changes requiring restart
            if (e.affectsConfiguration('duckyscript.lsp') && !e.affectsConfiguration('duckyscript.lsp.enable')) {
                window.showInformationMessage(
                    'DuckyScript: Some LSP feature settings require reloading the window to take effect.',
                    'Reload Window'
                ).then(selection => {
                    if (selection === 'Reload Window') {
                        commands.executeCommand('workbench.action.reloadWindow');
                    }
                });
            }
        })
    );

    // Register compile command
    const compileCommand = commands.registerCommand('duckyscript.compile', async () => {
        const editor = window.activeTextEditor;
        if (!editor) {
            window.showErrorMessage('No active file');
            return;
        }

        const document = editor.document;
        if (document.languageId !== 'duckyscript') {
            window.showErrorMessage('Current file is not DuckyScript');
            return;
        }

        // Save first (if auto-save is enabled)
        const autoSave = workspace.getConfiguration('duckyscript').get('compiler.autoSave', true);
        if (autoSave) {
            await document.save();
        }

        const filePath = document.uri.fsPath;
        const duckCliPath = findDuckCli();

        if (!duckCliPath) {
            window.showErrorMessage(
                'Duck CLI not found. Please install it or configure the path in settings.',
                'Install Instructions',
                'Configure Path'
            ).then(selection => {
                if (selection === 'Install Instructions') {
                    commands.executeCommand('vscode.open', Uri.parse('https://github.com/Greenstorm5417/duck-tools#installation'));
                } else if (selection === 'Configure Path') {
                    commands.executeCommand('workbench.action.openSettings', 'duckyscript.cli.path');
                }
            });
            return;
        }

        // Get or create terminal
        let terminal = window.activeTerminal;
        if (!terminal) {
            terminal = window.createTerminal('DuckyScript Compiler');
        }
        
        // Show terminal if configured
        const showTerminal = workspace.getConfiguration('duckyscript').get('compiler.showTerminal', true);
        if (showTerminal) {
            terminal.show();
        }
        
        // Detect shell type from environment or default shell
        // Check the SHELL environment variable or use platform defaults
        const platform = process.platform;
        let command: string;
        
        // Try to detect shell from the terminal's name or creation options
        const terminalName = terminal.name.toLowerCase();
        
        if (terminalName.includes('powershell') || terminalName.includes('pwsh') || 
            (platform === 'win32' && !terminalName.includes('bash') && !terminalName.includes('cmd'))) {
            // PowerShell (default on Windows): Use call operator & for quoted paths
            command = `& "${duckCliPath}" build -i "${filePath}"`;
        } else if (terminalName.includes('cmd') || terminalName.includes('command prompt')) {
            // CMD: Direct execution with quotes
            command = `"${duckCliPath}" build -i "${filePath}"`;
        } else if (terminalName.includes('bash') || terminalName.includes('zsh') || terminalName.includes('sh') || platform !== 'win32') {
            // Bash/Zsh/Unix shells: Direct execution with quotes
            command = `"${duckCliPath}" build -i "${filePath}"`;
        } else {
            // Default: Try PowerShell syntax (most common on Windows)
            command = `& "${duckCliPath}" build -i "${filePath}"`;
        }
        
        terminal.sendText(command);
    });

    context.subscriptions.push(compileCommand);
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
