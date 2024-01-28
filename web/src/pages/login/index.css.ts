import { globalStyle, style } from '@vanilla-extract/css';

import { colors } from 'src/styles/colors.css.ts';
import { link } from 'src/styles/forms.css.ts';

export const page = style([
	{
		display: 'grid',
		placeContent: 'center',
		gridTemplateColumns: '16rem',
		rowGap: '1rem',
	},
]);

export const form = style([
	{
		display: 'grid',
		rowGap: '1rem',
	},
]);

export const actions = style([
	{
		display: 'grid',
		gridAutoFlow: 'column',
		justifyContent: 'end',
		alignItems: 'center',
		gap: '1rem',
	},
]);

export const alert = style([
	{
		padding: '1rem',
		backgroundColor: colors.verdant[0],
		borderRadius: 4,
		minWidth: 0,
	}
])

globalStyle(`${page} ${link}`, {
	borderBottomColor: colors.verdant[90],
});
