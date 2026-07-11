import * as vscode from 'vscode';
import config from '../config';
import { dirname } from 'path';
import { ControllerMessage, EditorMessage, TreeNodeId } from './sharedTypes';
import HachimiIpc from '../core/hachimiIpc';
import { ZOKUZOKU_DIR } from '../defines';

export class EditorBase {
    subscribedPath: TreeNodeId[] = [];
    protected disposables: vscode.Disposable[] = [];

    constructor(
        protected readonly context: vscode.ExtensionContext
    ) { }

    protected getHtmlForWebview(_webview: vscode.Webview): string {
        return "";
    }

    protected setupWebview(
        webviewPanel: vscode.WebviewPanel,
        extraLocalResourceRoots: vscode.Uri[] = []
    ): vscode.Disposable[] {
        const panelDisposables: vscode.Disposable[] = [];
        const webview = webviewPanel.webview;
        const gameDataDir = config().get<string>("gameDataDir");
        const customFont = config().get<string>("customFont");

        webview.options = {
            enableScripts: true,
            localResourceRoots: [
                this.context.extensionUri,
                vscode.Uri.file(ZOKUZOKU_DIR),
                ...(gameDataDir ? [vscode.Uri.file(gameDataDir)] : []),
                ...(customFont ? [vscode.Uri.file(dirname(customFont))] : []),
                ...extraLocalResourceRoots
            ]
        };

        webview.html = this.getHtmlForWebview(webview);

        const postMessage = (message: ControllerMessage) => {
            webviewPanel.webview.postMessage(message);
        };

        webview.onDidReceiveMessage((message: EditorMessage) => {
            switch (message.type) {
                case "showMessage": {
                    const text = vscode.l10n.t(message.content, message.content);
                    switch (message.messageType) {
                        default:
                        case "info":
                            vscode.window.showInformationMessage(text);
                            break;
                        case "warning":
                            vscode.window.showWarningMessage(text);
                            break;
                        case "error":
                            vscode.window.showErrorMessage(text);
                            break;
                    }
                    break;
                }

                case "subscribePath":
                    this.subscribedPath = message.path;
                    break;

                case "callHachimiIpc":
                    HachimiIpc.callWithProgress(message.command)
                        .catch(e => vscode.window.showErrorMessage(String(e)));
                    break;

                case "showInputBox":
                    vscode.window.showInputBox({
                        placeHolder: vscode.l10n.t(message.placeholder, message.placeholder)
                    }).then(result => postMessage({
                        type: "showInputBoxResult",
                        id: message.id,
                        result
                    }));
                    break;
            }
        }, null, panelDisposables);

        webviewPanel.onDidDispose(() => {
            while (panelDisposables.length) {
                panelDisposables.pop()?.dispose();
            }
        }, null, panelDisposables);

        return panelDisposables;
    }

    static sendTranslationMap(
        postMessage: (msg: ControllerMessage) => void,
        ast: jsonToAst.ValueNode,
        astObjectsProps: Map<jsonToAst.ObjectNode, { [key: string]: jsonToAst.PropertyNode }>
    ) {
        const map: { [pathStr: string]: string[] } = {};
        if (ast.type === "Object") {
            for (const [key, prop] of Object.entries(astObjectsProps.get(ast) ?? {})) {
                if (prop.value.type === "Literal" && typeof prop.value.value === "string") {
                    map[key] = [prop.value.value];
                } else if (prop.value.type === "Object") {
                    for (const [subKey, subProp] of Object.entries(astObjectsProps.get(prop.value) ?? {})) {
                        if (subProp.value.type === "Literal" && typeof subProp.value.value === "string") {
                            map[`${key}/${subKey}`] = [subProp.value.value];
                        }
                    }
                }
            }
        }
        postMessage({ type: "setTranslationMap", map });
    }

    dispose() {
        while (this.disposables.length) {
            this.disposables.pop()?.dispose();
        }
    }
}
