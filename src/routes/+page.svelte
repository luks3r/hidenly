<script lang="ts">
	import HideCard from '$lib/cards/HideCard.svelte';
	import UnhideCard from '$lib/cards/UnhideCard.svelte';
	import { TabGroup, Tab } from '@skeletonlabs/skeleton';

	let tabSet: number = 0;
	let hiddenType: 'text' | 'image' = 'text';
</script>

<div class="container p-10 space-y-4">
	<TabGroup>
		<Tab bind:group={tabSet} name="hideTab" value={0}>Hide</Tab>
		<Tab bind:group={tabSet} name="unhideTab" value={1}>Unhide</Tab>

		<svelte:fragment slot="panel">
			<span>Hidden Type</span>
			<select class="select" bind:value={hiddenType}>
				<option value="text">Text</option>
				<option value="image">Image</option>
			</select>
			<br /><br />
			{#if tabSet === 0}
				<HideCard {hiddenType} />
			{:else if tabSet === 1}
				<UnhideCard {hiddenType} />
			{/if}
		</svelte:fragment>
	</TabGroup>
</div>
