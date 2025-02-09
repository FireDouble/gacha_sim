<button {disabled} {onclick}
class="
    transition-colors w-full py-2 px-4 bg-blue-400 text-white rounded-lg flex items-center justify-center
    { disabled ? "cursor-not-allowed opacity-50" : "hover:bg-blue-500"}
"
>
    {text}
</button>

<script>
    let { disabled = true, text = '', onclick} = $props();    
</script>