/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			keyframes: {
				fall: {
					'0%': { transform: 'translateY(-100%)' },
					'100%': { transform: 'translateY(1000%)' }
				}
			},
			animation: {
				fall: 'fall 2s ease-in-out infinite'
			}
		}
	},
	plugins: []
};
