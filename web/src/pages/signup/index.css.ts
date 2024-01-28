import { globalStyle, style } from '@vanilla-extract/css';

import { colors } from 'src/styles/colors.css.ts';
import { link } from 'src/styles/forms.css.ts';

export const page = style([
	{
		display: 'grid',
		placeContent: 'center',
	},
]);

export const form = style([
	{
		width: '16rem',
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

globalStyle(`${page} ${link}`, {
	borderBottomColor: colors.verdant[90],
});
