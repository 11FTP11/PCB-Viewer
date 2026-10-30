<script>
    var { children } = $props();
</script>
<style>
    :global(.gutter) {
        background-color: whitesmoke;
        background-repeat: no-repeat;
        background-position: center; 
        transition: background-color 0.2s ease; 
        z-index: 10; 
    }

    :global(.gutter:hover) {
        background: #83D03F;
    }

    :global(.gutter.gutter-horizontal) {
        cursor: col-resize;
    }

    :global(.gutter.gutter-vertical) {
        cursor: row-resize;
    }
</style>
{@render children()}