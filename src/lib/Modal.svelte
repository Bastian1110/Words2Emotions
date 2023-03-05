<script>
	import { slide } from 'svelte/transition';

	let isOpen = true;
</script>

<button class="px-4 py-2 bg-blue-400 text-white rounded-xl" on:click={() => (isOpen = !isOpen)}>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		fill="none"
		viewBox="0 0 24 24"
		stroke-width="1.5"
		stroke="currentColor"
		class="w-8 h-8"
	>
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z"
		/>
	</svg>
</button>

<div
	class="fixed inset-0 z-10 overflow-y-auto"
	style="display: {isOpen ? 'block' : 'none'};"
	transition:slide={{ duration: 1000 }}
>
	<div class="flex items-center justify-center min-h-screen">
		<div class="bg-white rounded-lg shadow-lg px-6 py-4">
			<h2 class="text-lg font-bold mb-4">Hello There!</h2>
			<p class="mb-4">Instructions</p>
			<button class="px-4 py-2 bg-blue-500 text-white" on:click={() => (isOpen = false)}
				>Close Modal</button
			>
		</div>
	</div>
</div>
