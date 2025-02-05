<main class="flex flex-col justify-center items-center m-2">
    <Calculator
    bind:pulls bind:simulations
    bind:character_pity bind:character_copies bind:character_guarantee
    bind:weapon_pity bind:weapon_copies bind:weapon_guarantee
    disabled={disabled}
    />
</main>


<script>
    import Calculator from "./components/Calculator.svelte";

    let pulls = $state(0);
    let simulations = $state(10000);

    let character_pity = $state(0);
    let character_copies = $state(0);
    let character_guarantee = $state(false);

    let weapon_pity = $state(0);
    let weapon_copies = $state(0);
    let weapon_guarantee = $state(false);

    let disabled = $derived(!(
        pulls > 0 &&
        character_pity >= 0 &&
        weapon_pity >= 0 &&
        simulations > 0 &&
        (character_copies > 0 || weapon_copies > 0)
    ))


    $inspect(pulls);
    $inspect(simulations);

    $inspect(character_pity);
    $inspect(character_copies);
    $inspect(character_guarantee);

    $inspect(weapon_pity);
    $inspect(weapon_copies);
    $inspect(weapon_guarantee);
</script>