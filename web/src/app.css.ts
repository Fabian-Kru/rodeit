import { style } from '@vanilla-extract/css';
import { colors } from './styles/colors.css';
import { lg } from './styles/screens.css';

export const nav = style([
	{
		display: 'grid',
		backgroundColor: colors.verdant[90],
		color: colors.verdant[30],
		boxShadow: `0 4px 0 0 ${colors.verdant[0]}`,
		zIndex: 1,
	},
	lg({
		paddingLeft: '16rem',
		paddingRight: '2rem',
	}),
]);

export const nav_ul = style([
	{
		display: 'grid',
		gridAutoFlow: 'column',
		gridTemplateColumns: 'minmax(0, 1fr) 16rem',
	},
]);

export const nav_pages = style([
	{
		height: '100%',
		display: 'grid',
		gridAutoFlow: 'column',
		columnGap: '2rem',
		justifyContent: 'start',
	},
]);

export const nav_user_actions = style([
	{
		height: '100%',
		display: 'grid',
		gridAutoFlow: 'column',
		columnGap: '2rem',
		justifyContent: 'end',
	},
]);

export const nav_link_container = style([
	{
		display: 'grid',
		alignContent: 'center',
	},
]);

export const nav_link = style([
	{
		display: 'block',
		paddingTop: '0.25rem',
		paddingBottom: '0.5rem',
		borderBottomWidth: 2,
		borderBottomStyle: 'solid',
		borderBottomColor: colors.verdant[30],
		':hover': {},
	},
]);
