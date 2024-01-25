import { createGlobalTheme } from '@vanilla-extract/css';

// spacing
// 2px -> 0.125rem
// 4px -> 0.25rem
// 6px -> 0.375rem
// 10px -> 0.625rem
// 16px -> 1rem
// 26px -> 1.625rem
// 42px -> 2.625rem
// 68px -> 4.25rem
// 110px -> 6.875rem
// 178px -> 11.125rem
// 288px -> 18rem
// 466px -> 29.125rem
// 754px -> 47.125rem
// 1220px -> 76.25rem
// 1974px -> 123.375rem
// 3194px -> 199.625rem

export const colors = createGlobalTheme(':root', {
	white: 'oklch(100% 0 0)',
	verdant: 'oklch(85% 0.02 130)',
	lemon: 'oklch(95% 0.2 110)',
	banana: 'oklch(85% 0.2 90)',
});
