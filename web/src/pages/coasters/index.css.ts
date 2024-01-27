import { createVar, style } from '@vanilla-extract/css';

import { colors } from 'src/styles/colors.css.ts';
import { lg } from 'src/styles/screens.css.ts';

export const page = style([
	lg({
		display: 'grid',
		gridTemplateColumns: '16rem 1fr',
	}),
]);

export const aside = style([
	{
		padding: '1rem',
	},
]);

export const aside_link = style([
	{
		display: 'block',
		padding: '1rem',
		borderRadius: 4,
		':hover': {
			backgroundColor: colors.verdant[0],
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
		padding: '1rem',
		display: 'grid',
		alignContent: 'start',
		rowGap: '1rem',
	},
	lg({
		paddingInline: 0,
		paddingBlock: '1rem',
		paddingRight: '16rem',
	}),
]);

export const coaster = style([
	{
		display: 'grid',
		gridAutoRows: 'max-content',
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

export const coaster_details_image = createVar();

export const coaster_details = style([
	{
		display: 'grid',
		gridTemplateRows: '1fr',
		alignContent: 'end',
		padding: '1rem',
		gap: '1rem',
		aspectRatio: '2 / 1',
		backgroundImage: `linear-gradient(to right top, ${colors.verdant[30]}, oklch(0% 0 0 / 0)), ${coaster_details_image}`,
		backgroundSize: 'cover',
		backgroundPosition: 'center',
		borderColor: colors.verdant[90],
		borderRadius: 4,
		borderStyle: 'solid',
		borderWidth: 2,
		boxShadow: `0 4px 0 0 ${colors.verdant[0]}`,
	},
]);

export const coaster_actions = style([{}]);
