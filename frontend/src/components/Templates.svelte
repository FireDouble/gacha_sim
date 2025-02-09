<script module>
    export function get_templates() {
        const templates = $state([
            {
                name: "Honkai: Star Rail",
                settings: [
                    {
                        base_rate: 0.006,
                        limited_rate: 0.5,
                        limited_options: 1,
                        guaranteed_after: 1,
                        hard_pity: 90,
                        soft_pity: 74,
                        soft_pity_increment: 0.06,
                    },
                    {
                        base_rate: 0.006,
                        limited_rate: 0.75,
                        limited_options: 1,
                        guaranteed_after: 1,
                        hard_pity: 80,
                        soft_pity: 64,
                        soft_pity_increment: 0.06,
                    }
                ]
            },
            {
                name: "Zenless Zone Zero",
                settings: [
                    {
                        base_rate: 0.006,
                        limited_rate: 0.5,
                        limited_options: 1,
                        guaranteed_after: 1,
                        hard_pity: 90,
                        soft_pity: 74,
                        soft_pity_increment: 0.06,
                    },
                    {
                        base_rate: 0.006,
                        limited_rate: 0.75,
                        limited_options: 1,
                        guaranteed_after: 1,
                        hard_pity: 80,
                        soft_pity: 64,
                        soft_pity_increment: 0.06,
                    }
                ]
            },
            {
                name: "Genshin Impact",
                settings: [
                    {
                        base_rate: 0.006,
                        limited_rate: 0.5,
                        limited_options: 1,
                        guaranteed_after: 1,
                        hard_pity: 90,
                        soft_pity: 74,
                        soft_pity_increment: 0.06,
                    },
                    {
                        base_rate: 0.006,
                        limited_rate: 0.75,
                        limited_options: 2,
                        guaranteed_after: 1,
                        hard_pity: 80,
                        soft_pity: 64,
                        soft_pity_increment: 0.06,
                    }
                ]
            },
            {
                name: "Wuthering Waves",
                settings: [
                    {
                        base_rate: 0.008,
                        limited_rate: 0.5,
                        limited_options: 1,
                        guaranteed_after: 1,
                        hard_pity: 80,
                        soft_pity: 64,
                        soft_pity_increment: 0.06,
                    },
                    {
                        base_rate: 0.008,
                        limited_rate: 0.75,
                        limited_options: 1,
                        guaranteed_after: 0,
                        hard_pity: 80,
                        soft_pity: 64,
                        soft_pity_increment: 0.06,
                    }
                ]
            },
        ]);

        

        return templates
    }

    export function get_default_targets() {
        let targets = $state([
            {
                copies: 0,
                pity: 0,
                is_guaranteed: false,
                losses: 0
            },
            {
                copies: 0,
                pity: 0,
                is_guaranteed: false,
                losses: 0
            }
        ]);

        return targets;
    }
</script>