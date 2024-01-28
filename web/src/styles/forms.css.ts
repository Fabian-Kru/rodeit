import { style } from '@vanilla-extract/css';

import { label_large } from 'src/styles/atomic/fonts.css.ts';
import { colors } from 'src/styles/colors.css.ts';

export const input_text = style([
	label_large,
	{
		minWidth: '0',
		width: 'auto',
		height: '42px',
		paddingInline: '1rem',
		borderRadius: 4,
		borderWidth: 2,
		borderStyle: 'solid',
		borderColor: colors.verdant[90],
		boxShadow: `0 4px 0 0 ${colors.verdant[0]}`,
		color: colors.verdant[30],
		backgroundColor: colors.verdant[90],
		'::placeholder': {
			color: colors.verdant[60],
		},
	},
]);

export const link = style([
	{
		display: 'block',
		paddingTop: '0.25rem',
		paddingBottom: '0.5rem',
		borderBottomWidth: 2,
		borderBottomStyle: 'solid',
		borderBottomColor: colors.verdant[60],
		selectors: {
			'&[aria-current="page"], &:hover, &:focus-visible': {
				borderBottomColor: colors.verdant[30],
			},
		},
	},
]);

export const btn = style([
	label_large,
	{
		height: '42px',
		paddingInline: '1rem',
		backgroundColor: colors.verdant[100],
		color: colors.verdant[30],
		borderRadius: 4,
		borderWidth: 2,
		borderStyle: 'solid',
		borderColor: colors.verdant[90],
		boxShadow: `0 4px 0 0 ${colors.verdant[0]}`,

		selectors: {
			'&:hover, &:focus-visible': {
				backgroundColor: colors.verdant[90],
			},
		},
	},
]);
