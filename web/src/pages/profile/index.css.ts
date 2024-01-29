import { globalStyle, style } from '@vanilla-extract/css';

import { colors } from 'src/styles/colors.css.ts';

export const form = style([
	{
		display: 'grid',
		rowGap: '1rem',
		maxWidth: '24rem',
	},
]);

globalStyle(`${form} input`, {
	marginBottom: '1rem',
});

globalStyle(`form button[type=submit]`, {
	justifySelf: 'start',
});

export const alert = style([
	{
		maxWidth: '24rem',
		padding: '1rem',
		backgroundColor: colors.verdant[0],
		borderRadius: 4,
		minWidth: 0,
	},
]);
