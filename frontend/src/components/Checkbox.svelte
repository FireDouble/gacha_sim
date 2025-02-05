<label
class="select-none cursor-pointer h-[42px] border wfull px-3 py-2 rounded-md border-gray-600 transition-colors bg-gray-700"
>
    <div class="flex flex-row">
        <input {id} type="checkbox" bind:checked/>
        <p class="ml-2">{text}</p>
    </div>
</label>

<script>
    let { id, text = '', checked = $bindable() } = $props();
</script>