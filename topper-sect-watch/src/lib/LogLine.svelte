<script lang="ts">

    let { line } = $props();

    let sections = $derived.by(() => {
        const parts: {text: string, color: string}[] = [];
        let remaining = line;
        let current = '';
        let color = 'white';
        while (remaining.length > 0) {
            // Parse a string like: <color>text<color2>text2 -> [{text, color}, {text2, color2}]
            const match = remaining.match(/<([a-zA-Z0-9#]+)>/);
            if (match) {
                if (match.index! > 0) {
                    current += remaining.slice(0, match.index);
                }
                if (current.length > 0) {
                    parts.push({ text: current, color });
                    current = '';
                }
                color = match[1];
                remaining = remaining.slice(match.index! + match[0].length);
            } else {
                current += remaining;
                remaining = '';
                if (current.length > 0) {
                    parts.push({ text: current, color });
                }
            }
        }
        return parts;
    });
</script>

<span class="font-mono text-sm whitespace-pre-wrap block">
    {#each sections as section}
        <span style="color: {section.color}">{section.text}</span>
    {/each}
</span>