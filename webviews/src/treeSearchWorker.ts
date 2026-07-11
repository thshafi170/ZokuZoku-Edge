import type { ITreeNode, TreeNodeId } from "./sharedTypes"

export interface SearchOptions {
    caseSensitive: boolean,
    regex: boolean,
    searchInContent: boolean,
    excludeCategoryNames: boolean,
    searchById: boolean,
    searchInTranslation: boolean
}

export type SearchRequest = {
    type: "start",
    nodes: ITreeNode[],
    query: string,
    options: SearchOptions,
    translationMap: { [pathStr: string]: string[] }
}

export type SearchResponse = {
    type: "result",
    node: ITreeNode,
    parents: { id: TreeNodeId, name: string }[]
} | {
    type: "end"
};

type Matcher = (input: string) => boolean;

function createMatcher(query: string, options: SearchOptions): Matcher {
    if (options.regex) {
        return input => new RegExp(query, options.caseSensitive ? undefined : "i").test(input);
    }

    if (options.caseSensitive) {
        return input => input.includes(query);
    }

    query = query.toLowerCase();
    return input => input.toLowerCase().includes(query);
}

function treeSearch(
    nodes: ITreeNode[], matchString: Matcher, options: SearchOptions,
    query: string,
    translationMap: { [pathStr: string]: string[] },
    parents: { id: TreeNodeId, name: string }[] = []
) {
    function pushResult(node: ITreeNode) {
        let message: SearchResponse = {
            type: "result",
            node,
            parents
        };
        postMessage(message);
    }

    for (const node of nodes) {
        let nameMatch = matchString(node.name) || (node.type === "entry" && node.content[0] && matchString(node.content[0].content));
        switch (node.type) {
            case "category": {
                let categoryIdMatch = false;
                if (options.searchById) {
                    categoryIdMatch = String(node.id).startsWith(query.trim());
                }
                if ((nameMatch && !options.excludeCategoryNames) || categoryIdMatch) {
                    pushResult({
                        ...node,
                        children: []
                    });
                }
                treeSearch(
                    node.children, matchString, options, query,
                    translationMap,
                    [...parents, { id: node.id, name: node.name }]
                );
                break;
            }

            case "entry":
                let contentMatch = false;
                if (options.searchInContent) {
                    for (const slot of node.content) {
                        if (matchString(slot.content)) {
                            contentMatch = true;
                            break;
                        }
                    }
                }
                let contentIdMatch = false;
                if (options.searchById) {
                    contentIdMatch = String(node.id).startsWith(query.trim());
                }
                let translationMatch = false;
                if (options.searchInTranslation) {
                    const pathStr = [...parents.map(p => p.id), node.id].join("/");
                    const translations = translationMap[pathStr] || translationMap[node.id];
                    if (translations) {
                        for (const t of translations) {
                            if (t && matchString(t)) {
                                translationMatch = true;
                                break;
                            }
                        }
                    }
                }
                if (nameMatch || contentMatch || contentIdMatch || translationMatch) {
                    pushResult(node);
                }
                break;
        }
    }
}

addEventListener("message", e => {
    const message: SearchRequest = e.data;
    switch (message.type) {
        case "start": {
            const matcher = createMatcher(message.query, message.options);
            treeSearch(message.nodes, matcher, message.options, message.query, message.translationMap);

            const res: SearchResponse = { type: "end" };
            postMessage(res);
            close();
            break;
        }
    }
});