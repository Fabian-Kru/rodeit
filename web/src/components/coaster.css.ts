import { createVar, style } from '@vanilla-extract/css';

import { colors } from 'src/styles/colors.css.ts';
import { durations, ease } from 'src/styles/motion.css.ts';

export const coaster_details_image = createVar();

export const coaster_details = style([
	{
		display: 'grid',
		gridTemplateRows: '1fr',
		alignContent: 'end',
		padding: '1rem',
		gap: '1rem',
		height: '12rem',
		backgroundImage: `linear-gradient(to right top, ${colors.verdant[30]}, oklch(0% 0 0 / 0)), ${coaster_details_image}`,
		backgroundSize: 'cover',
		backgroundPosition: 'center',
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

export const coaster_details_meta = style([
	{
		alignSelf: 'start',
	},
]);

export const coaster_details_park = style([
	{
		display: 'grid',
		gap: '0.5rem',
		justifyContent: 'start',
		alignItems: 'center',
		gridAutoFlow: 'column',
	},
]);
