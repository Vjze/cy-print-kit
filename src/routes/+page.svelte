<script lang="ts">
	import MainUi from '../lib/components/MainUi.svelte';
</script>
<main>
	<MainUi />
	<slot></slot>
</main>

<style>
	:global(body) {
    margin: 0;
    padding: 0;
    background-image: url('../assets/pic.png');
    background-size: cover; /* 或使用 contain，根据你的需要 */
    background-position: center;
    background-repeat: no-repeat;
    width: 100%;
    height: 600px;
    overflow: hidden;
  }
	main{
		/* height: 600px; */
		/* background: #ccc; */
		border-radius: 10px;
	}
</style>