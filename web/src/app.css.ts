import { style } from '@vanilla-extract/css';
import { colors } from './styles/colors.css';

export const nav = style([
	{
		backgroundColor: colors.verdant[90],
		color: colors.verdant[30],
		boxShadow: `0 4px 0 0 ${colors.verdant[0]}`,
		zIndex: 1,
	},
]);

export const nav_ul = style([
	{
		display: 'grid',
	},
]);
