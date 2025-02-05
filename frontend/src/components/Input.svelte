<div class="w-full">
    <input {type} {...props} bind:value
    class="w-full px-3 py-2 border rounded-md border-gray-600 transition-colors bg-gray-700 focus:outline-none focus:border-blue-300"
    />

    <div
    class="
        flex items-center gap-1 mt-1 text-sm text-gray-500
        {!label && "hidden"}
    "
    >
        {label}
        <Tooltip message={tooltip} />
    </div>
</div>

<script>
    import Tooltip from "./Tooltip.svelte";

    let { type, label, tooltip = '', value = $bindable(), ...props} = $props();
</script>

