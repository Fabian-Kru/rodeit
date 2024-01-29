import { globalStyle, style } from '@vanilla-extract/css';

export const form = style([
	{
		display: 'grid',
		rowGap: '1rem',
		maxWidth: '20rem',
	},
]);

globalStyle(`${form} label`, {
	paddingTop: '1rem',
});

globalStyle(`${form} button[type=submit]`, {
	justifySelf: 'end',
});
