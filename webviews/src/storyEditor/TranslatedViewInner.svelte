<script lang="ts">
    import { createEventDispatcher, onMount, onDestroy } from "svelte";
    import { currentPath, currentTextSlots } from "../stores";
    import type { IPanelAction } from "../types";
    import type { ControllerMessage } from "../sharedTypes";
    import { translatedSlotProps } from "../utils";
    import TextSlot from "../lib/TextSlot.svelte";
    import GenericSlots from "../lib/GenericSlots.svelte";
    import StorySplitView from "./StorySplitView.svelte";
    import { translatedPreview } from "./stores";
    import * as l10n from "@vscode/l10n";

    const dispatch = createEventDispatcher<{
        updateActions: IPanelAction[];
        updateCharCount: number;
    }>();

    const preview = translatedPreview;
    const actions: IPanelAction[] = [
        {
            icon: "comment",
            tooltip: l10n.t("Dialogue preview"),
            onClick: () => $preview = $preview == "dialogue" ? null : "dialogue"
        },
        {
            icon: "book",
            tooltip: l10n.t("Story preview"),
            onClick: () => $preview = $preview == "story" ? null : "story"
        }
    ];

    const placeholder = l10n.t("Type your translation here...");

    let charCount: number = 0;

    $: dispatch("updateActions", actions);
    $: dispatch("updateCharCount", charCount);

    function longestLineLength(value: string): number {
        const lines = value.split(/\n|\\n/);
        let max = 0;
        for (const line of lines) {
            if (line.length > max) max = line.length;
        }
        return max;
    }

    function recount() {
        let total = 0;
        const elements = document.querySelectorAll<HTMLInputElement | HTMLTextAreaElement>(
            "input:not([readonly])[data-slot-type='1'], textarea:not([readonly])[data-slot-type='1']",
        );
        for (const el of elements) {
            if (el.value) total += longestLineLength(el.value);
        }
        charCount = total;
    }

    function onDocumentInput() {
        recount();
    }

    onMount(() => {
        document.addEventListener("input", onDocumentInput);
    });

    onDestroy(() => {
        document.removeEventListener("input", onDocumentInput);
    });

    function onMessage(e: MessageEvent<ControllerMessage>) {
        const message = e.data;
        if (
            message.type === "setTextSlotContent" &&
            message.entryPath.join("/") === $currentPath.join("/")
        ) {
            requestAnimationFrame(recount);
        }
    }

    $: $currentPath, (() => {
        charCount = 0;
        (document.activeElement as HTMLElement | null)?.blur();
    })();
</script>

<svelte:window on:message={onMessage} />

<StorySplitView preview={$preview} translated>
    <GenericSlots>
        {#key $currentTextSlots}
            {#each $currentTextSlots as slot, index}
                <TextSlot {...translatedSlotProps(slot)} {index} entryPath={$currentPath} {placeholder} />
            {/each}
        {/key}
    </GenericSlots>
</StorySplitView>