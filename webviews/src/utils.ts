import { wrapText } from "hachimi_lib";
import type { ControllerMessage, ITextSlot, ITreeNode, StoryEditorConfig, TreeNodeId } from "./sharedTypes";
import { currentNav, currentPath, currentTextSlots, selectedNodes } from "./stores";
import { vscode } from "./vscode";

export function findNodeByPath(path: TreeNodeId[], nodes: ITreeNode[]): [ITreeNode, ITreeNode[]] | null {
    if (!path.length) return null;

    for (const node of nodes) {
        if (node.id == path[0]) {
            if (path.length == 1) {
                return [ node, nodes ];
            }
            else if (node.type == "category") {
                return findNodeByPath(path.slice(1), node.children)
            }
            else {
                return null;
            }
        }
    }

    return null;
}

export function getNodeTlContent(entryPath: TreeNodeId[], contentCount: number): Promise<(string | null)[]> {
    return new Promise(resolve => {
        const res: (string | null)[] = [];
        const setIndexes = new Set<number>();
        const onMessage = (e: MessageEvent) => {
            const message: ControllerMessage = e.data;
            if (message.type == "setTextSlotContent" &&
                message.entryPath.join("/") == entryPath.join("/"))
            {
                res[message.index] = message.content;
                setIndexes.add(message.index);
                if (setIndexes.size == contentCount) {
                    resolve(res);
                    window.removeEventListener("message", onMessage);
                }
            }
        };
        window.addEventListener("message", onMessage);
        for (let i = 0; i < contentCount; ++i) {
            vscode.postMessage({
                type: "getTextSlotContent",
                entryPath,
                index: i
            });
        }
    });
}

export function gotoNode(node: ITreeNode, path: TreeNodeId[]) {
    if (node.type != "entry") return;
    selectedNodes.update(() => ({ [path.join("/")]: node.content.length }));
    currentPath.update(() => path);
    currentTextSlots.update(() => node.content);
    currentNav.update(() => ({
        next: node.next,
        prev: node.prev
    }));
}

export function translatedSlotProps(slot: ITextSlot) {
    return {
        ...slot,
        content: null,
        postContent: true
    }
}

export function makeContentDisplayValue(
    value: string | null, lineWidth: number, config: StoryEditorConfig | null, readonly: boolean
) {
    const val = value?.replace(/\\n/g, "\n").replace(/<n>/gi, " ") ?? "";
    return config?.noWrap === false && config.lineWidthMultiplier ?
        wrapText(val, lineWidth, config.lineWidthMultiplier).join("\n") :
        val;
}

export function highlightTags(text: string | null): string {
    if (!text) return "";
    const escaped = text
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;");

    // highlight game tags and line-break markers
    return escaped.replace(
        /(&lt;\/?[a-zA-Z_][\w]*(?:\s*=\s*[^&<>]*)?\&gt;|\\n|\\\\n)/g,
        '<span class="tag-highlight">$1</span>',
    );
}