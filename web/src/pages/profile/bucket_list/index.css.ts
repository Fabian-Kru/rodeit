import { style } from '@vanilla-extract/css';

export const coasters = style([
	{
		display: 'grid',
		rowGap: '2rem',
	},
]);

export const coaster = style([
	{
		display: 'grid',
		columnGap: '1rem',
		gridTemplateColumns: 'minmax(0, 1fr) 42px',
	},
]);

export const coaster_actions = style([
	{
		display: 'grid',
		rowGap: '1rem',
		alignContent: 'center',
	},
]);
