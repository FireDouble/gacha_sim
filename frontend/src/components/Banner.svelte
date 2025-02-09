<Title text={title}/>
<section class="flex gap-5">
    <Input label="Pity" type="number" placeholder="0" autocomplete="off" bind:value={pity} />
    <Input label="Copies" type="number" placeholder="0" autocomplete="off" bind:value={copies} />
    {#if guaranteed_after != 0}
        {#if guaranteed_after == 1}
            <Checkbox id="weapon" text="Guaranteed" bind:checked={is_guaranteed} /> 
        {:else}
            <Input label="Losses" type="number" placeholder="0" autocomplete="off" bind:value={losses} />
        {/if}
    {/if}
</section>

<script>
    import Title from "./Title.svelte";
    import Checkbox from "./Checkbox.svelte";
    import Input from "./Input.svelte";

    let { title, pity = $bindable(0), copies = $bindable(0), is_guaranteed = $bindable(false), losses = $bindable(0), guaranteed_after} = $props();
</script>