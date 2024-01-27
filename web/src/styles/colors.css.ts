import { createGlobalTheme } from '@vanilla-extract/css';

export const colors = createGlobalTheme(':root', {
	verdant: {
		0: 'oklch(0% 0 0)',
		30: 'oklch(30% 0.01 130)',
		90: 'oklch(90% 0.02 130)',
		100: 'oklch(100%, 0, 0)',
	},
	lemon: 'oklch(95% 0.2 110)',
	banana: 'oklch(85% 0.2 90)',
} as const);
