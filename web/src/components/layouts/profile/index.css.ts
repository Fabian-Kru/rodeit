import { style } from '@vanilla-extract/css';
import { colors } from 'src/styles/colors.css';

export const page = style([
	{
		display: 'grid',
		gridTemplateColumns: '16rem 1fr',
		gap: '2rem',
		paddingInline: '2rem',
	},
]);

export const aside = style([
	{
		display: 'grid',
		rowGap: '2rem',
		paddingBlock: '2rem',
	},
]);

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
		alignContent: 'start',
		rowGap: '2rem',
		paddingBlock: '2rem',
	},
]);

export const form = style([
	{
		alignSelf: 'end',
	},
]);
