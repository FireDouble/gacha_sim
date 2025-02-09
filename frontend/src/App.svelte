<Navbar bind:targets={app_state.targets} bind:settings={app_state.settings}/>

<main class="flex flex-col justify-center items-center m-2">
    <Calculator
    bind:targets={app_state.targets}
    bind:pulls={app_state.pulls} bind:simulations={app_state.simulations}
    {...app_state}
    disabled={disabled}
    on_calculate={send_simulate_request}
    />
</main>


<script>
    import Calculator from "./components/Calculator.svelte";
    import Navbar from "./components/Navbar.svelte"
    import { get_templates, get_default_targets } from "./components/Templates.svelte";

    let app_state = $state({
        pulls: 100,
        simulations: 10000,
        odds: null,
        body: null,

        targets: get_default_targets(),
        settings: get_templates()[0].settings,
    });

    let disabled = $derived(!(
        app_state.pulls > 0 &&
        app_state.targets[0].pity >= 0 &&
        app_state.targets[1].pity >= 0 &&
        app_state.simulations > 0 &&
        (app_state.targets[0].copies > 0 || app_state.targets[1].copies > 0)
    ))


    async function send_simulate_request() {
        const response = await fetch("http://localhost:3030/simulate", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            }, 
            body: JSON.stringify(app_state),
        });

        console.log(app_state)

        const data = await response.json();

        console.log(data.successfull_simulations);

        app_state.odds = ((data.successfull_simulations / app_state.simulations) * 100).toFixed(String(app_state.simulations/1000).length) + "%";
        app_state.body = `
            Is the propability of you obtaining
            ${app_state.targets[0].copies !== 0 ? `
                ${app_state.targets[0].copies} copy of the limited Character from the featured banner
            ` : ""}
            ${app_state.targets[1].copies !== 0 && app_state.targets[1].copies !== 0 ? " and " : ""}
            ${app_state.targets[1].copies !== 0 ? `
                ${app_state.targets[1].copies} copies of the limited Light Cone from the featured banned 
            ` : ""}

            if you were to do ${app_state.pulls} warps.
        `;
    }
</script>
