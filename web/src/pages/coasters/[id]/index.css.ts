import { createVar, globalStyle, style } from '@vanilla-extract/css';

import { colors } from 'src/styles/colors.css.ts';
import { durations, ease } from 'src/styles/motion.css.ts';

export const page = style([{}]);

export const header_image = createVar();

export const header = style([
	{
		display: 'grid',
		gridTemplateColumns: 'minmax(0, 48rem)',
		rowGap: '1rem',
		justifyContent: 'center',
		alignContent: 'end',
		padding: '3rem',
		aspectRatio: '4',
		backgroundImage: [
			`linear-gradient(to right top, ${colors.verdant[30]}, oklch(0% 0 0 / 0) 50%)`,
			header_image,
		].join(', '),
		backgroundSize: 'cover',
		backgroundPosition: 'center',
		borderBottomColor: colors.verdant[90],
		borderBottomStyle: 'solid',
		borderBottomWidth: 2,
	},
]);

export const park = style([
	{
		display: 'grid',
		columnGap: '1rem',
		gridTemplateColumns: 'max-content 1fr',
	},
]);

export const main = style([
	{
		display: 'grid',
		gridTemplateColumns: 'minmax(0, 48rem)',
		justifyContent: 'center',
		paddingBlock: '3rem',
		rowGap: '3rem',
	},
]);

export const actions = style([
	{
		display: 'grid',
		gridAutoFlow: 'column',
		justifyContent: 'start',
		columnGap: '1rem',
	},
]);

export const social = style([
	{
		display: 'grid',
		gridTemplateColumns: 'minmax(0, 1fr) minmax(0, 1fr)',
		columnGap: '2rem',
		alignItems: 'start',
	},
]);

export const social_section = style([
	{
		display: 'grid',
		rowGap: '1.5rem',
	},
]);

globalStyle(`${social_section} > p`, {
	paddingBottom: '1.5rem',
});

export const stats = style([
	{
		display: 'grid',
		gridTemplateColumns: 'minmax(0, 1fr) minmax(0, 1fr)',
		gridAutoRows: 'max-content',
		gap: '2rem',
		alignItems: 'start',
	},
]);

globalStyle(`${stats} li`, {
	display: 'grid',
	rowGap: '1rem',
	borderBottomColor: colors.verdant[60],
	borderBottomStyle: 'solid',
	borderBottomWidth: 2,
	paddingBottom: 24,
});

globalStyle(`${stats} li svg`, {
	stroke: colors.verdant[90],
});

export const records = style([
	{
		display: 'grid',
		rowGap: '1rem',
	},
]);

export const records_title = style([
	{
		display: 'grid',
		justifyContent: 'space-between',
		alignItems: 'end',
		gridAutoFlow: 'column',
	},
]);

globalStyle(`${records_title} > p`, {
	color: colors.verdant[60],
});

export const record = style([
	{
		display: 'grid',
		rowGap: '1rem',
		padding: '1rem',
		borderColor: colors.verdant[90],
		borderRadius: 4,
		borderStyle: 'solid',
		borderWidth: 2,
		boxShadow: `0 4px 0 0 ${colors.verdant[0]}`,
		transitionProperty: 'translate, boxShadow',
		transitionTimingFunction: ease.standard,
		transitionDuration: durations.short,
		selectors: {
			'&:hover, &:focus-visible': {
				translate: '0 -2px',
				boxShadow: `0 6px 0 0 ${colors.verdant[0]}`,
			},
		},
	},
]);
