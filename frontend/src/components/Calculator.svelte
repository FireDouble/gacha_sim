<div class="w-full max-w-md bg-gray-700 rounded-xl shadow-lg p-6 my-12 space-y-4">
    <Title text="Settings" />
    <section class="flex gap-5">
        <Input label="Pulls" type="number" placeholder="0" autocomplete="off" bind:value={pulls} />
        <Input label="Simulations" type="number" placeholder="0" autocomplete="off" bind:value={simulations} />
    </section>

    {#each targets as target, i}
        <Banner title={`Section ${i+1}`}
        bind:pity={target.pity} bind:copies={target.copies} bind:is_guaranteed={target.is_guaranteed} bind:losses={target.losses}
        guaranteed_after={settings[i].guaranteed_after}
        />
    {/each}


    <div class="space-y-6">
        <Button text="Calculate" {disabled} onclick={on_calculate} />
    </div>

    {#if odds !== null}
        <Results bind:title={odds} bind:body/>
    {/if}
</div>

<script>
    import Button from "./Button.svelte";
    import Banner from "./Banner.svelte";
    import Title from "./Title.svelte";
    import Input from "./Input.svelte";
    import Results from "./Results.svelte";

    let {
        pulls = $bindable(100),
        simulations = $bindable(10000),

        targets = $bindable([
            {
                copies: 0,
                pity: 0,
                is_guaranteed: false,
                losses: 0,
            },
            {
                copies: 0,
                pity: 0,
                is_guaranteed: false,
                losses: 0,
            }
        ]),

        settings = $bindable([{
            base_rate: 0.006,
            limited_rate: 0.5,
            limited_options: 1,
            guaranteed_after: 2,

            hard_pity: 90,
            soft_pity: 74,
            soft_pity_increment: 0.06,
        }]),


        odds = $bindable(null), body = $bindable(null),
        on_calculate, disabled
    } = $props();
</script>
