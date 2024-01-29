import { style } from '@vanilla-extract/css';

import { colors } from 'src/styles/colors.css.ts';

export const page = style([{}]);

export const header = style([
	{
		display: 'grid',
		gridTemplateColumns: 'minmax(0, 48rem)',
		rowGap: '1rem',
		justifyContent: 'center',
		alignContent: 'end',
		padding: '3rem',
		aspectRatio: '4',
		borderBottomColor: colors.verdant[90],
		borderBottomStyle: 'solid',
		borderBottomWidth: 2,
	},
]);

export const columns = style([
	{
		display: 'grid',
		gridTemplateColumns: 'minmax(0, 22.5rem) minmax(0, 22.5rem)',
		columnGap: '3rem',
		justifyContent: 'center',
		alignItems: 'start',
	},
]);

export const column = style([
	{
		display: 'grid',
		paddingBlock: '3rem',
		rowGap: '2rem',
	},
]);

export const coasters = style([
	{
		display: 'grid',
		rowGap: '1.5rem',
	},
]);
