<nav class="justify-center pt-4 pl-1 flex gap-6 bg-slate-800 shadow-md">
    {#each get_templates() as template, i}
        {#if i == selected}
            <button onclick={() => {update(i)}}
                class="inline-flex shrink-0 items-center gap-2 border-b-2 px-1 pb-4 text-sm font-medium text-sky-600 border-sky-500 cursor-pointer"
            >
                {template.name}
            </button>
        {:else}
            <button onclick={() => {update(i)}}
                class="
                    inline-flex shrink-0 items-center gap-2 border-b-2 px-1 pb-4 text-sm font-medium text-gray-500 border-transparent hover:border-gray-300 hover:text-gray-400
                    cursor-pointer
                "
            >
                {template.name}
            </button>
        {/if}
    {/each}
</nav>

<script>
    import { get_templates } from "./Templates.svelte";

    let { targets = $bindable(), settings = $bindable(null), selected = 0} = $props();

    async function update(i) {
        settings = get_templates()[i].settings;
        let t = [];
        for (let i = 0; i < settings.length; i++) {
            t.push(
                {
                    copies: 0,
                    pity: 0,
                    is_guaranteed: false,
                    losses: 0
                },
            );
        }
        targets = t;
        selected = i;
    }
</script>
