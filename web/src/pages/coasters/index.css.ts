import { globalStyle, style } from '@vanilla-extract/css';

import { colors } from 'src/styles/colors.css.ts';
import { lg } from 'src/styles/screens.css.ts';

export const page = style([
	lg({
		display: 'grid',
		gridTemplateColumns: '16rem 1fr',
		gap: '2rem',
		paddingInline: '2rem',
	}),
]);

export const aside = style([
	{
		display: 'grid',
		rowGap: '2rem',
		alignContent: 'start',
		paddingBlock: '2rem',
	},
]);

export const aside_form = style([
	{
		display: 'grid',
		rowGap: '1rem',
	},
]);

globalStyle(`${aside_form} button[type=submit]`, {
	justifySelf: 'end',
});

export const aside_link = style([
	{
		display: 'block',
		padding: '1rem',
		borderRadius: 4,
		selectors: {
			'&[aria-current="page"], &:hover, &:focus-visible': {
				backgroundColor: colors.verdant[0],
			},
		},
	},
]);

export const main = style([
	{
		overflowY: 'auto',
		display: 'grid',
	},
]);

export const coasters = style([
	{
		paddingBlock: '1rem',
		display: 'grid',
		alignContent: 'start',
		rowGap: '1rem',
	},
	lg({
		paddingInline: 0,
		paddingBlock: '2rem',
	}),
]);

export const coaster_actions = style([{}]);
