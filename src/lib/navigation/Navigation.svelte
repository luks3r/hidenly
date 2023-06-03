<script lang="ts">
	import { drawerStore } from '@skeletonlabs/skeleton';

	function drawerClose(): void {
		drawerStore.close();
	}

	let menus = [
		{ href: '/', label: 'Home' },
		{ href: '/about', label: 'About' },
		{ href: '/blog', label: 'Blog' },
		{ href: '/contact', label: 'Contact' }
	];
</script>

<nav class="list-nav p-4">
	<ul>
		{#each menus as menu}
			<li class="list-nav-item">
				<a href={menu.href} on:click={drawerClose}>{menu.label}</a>
			</li>
		{/each}
	</ul>
</nav>
