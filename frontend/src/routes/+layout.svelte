<script>
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<style>
	:global(body) {
		font-family: Verdana, Geneva, Tahoma, sans-serif;
		margin: 0;
		padding: 0;
	}
</style>
{@render children()}
