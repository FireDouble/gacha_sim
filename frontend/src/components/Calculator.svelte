<div class="w-full max-w-md bg-gray-700 rounded-xl shadow-lg p-6 my-12 space-y-4">
    <Title text="Settings" />
    <section class="flex gap-5">
        <Input label="pulls" type="number" placeholder="0" autocomplete="off" bind:value={pulls} />
        <Input label="Simulations" type="number" placeholder="0" autocomplete="off" bind:value={simulations} />
    </section>

    <Title text="Character"/>
    <section class="flex gap-5">
        <Input label="Pity" type="number" placeholder="0" autocomplete="off" bind:value={character_pity} />
        <Input label="Character" type="number" placeholder="0" autocomplete="off" bind:value={character_copies} />
        <Checkbox id="char" text="Guaranteed" bind:checked={character_guarantee} />
    </section>

    <Title text="Light Cone"/>
    <section class="flex gap-5">
        <Input label="Pity" type="number" placeholder="0" autocomplete="off" bind:value={weapon_pity} />
        <Input label="Light Cone" type="number" placeholder="0" autocomplete="off" bind:value={weapon_copies} />
        <Checkbox id="weapon" text="Guaranteed" bind:checked={weapon_guarantee} />
    </section>


    <div class="space-y-6">
        <Button text="Calculate" {disabled}/>
    </div>
</div>

<script>
    import Button from "./Button.svelte";
    import Checkbox from "./Checkbox.svelte";
    import Input from "./Input.svelte";
    import Title from "./Title.svelte";


    let {
        pulls = $bindable(0), simulations = $bindable(10000),
        character_pity = $bindable(0), character_copies = $bindable(0), character_guarantee = $bindable(false),
        weapon_pity = $bindable(0), weapon_copies = $bindable(0), weapon_guarantee = $bindable(false),
        disabled = $bindable(true)
    } = $props();
</script>