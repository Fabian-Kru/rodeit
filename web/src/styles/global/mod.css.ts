import { globalStyle } from '@vanilla-extract/css';

import './_preflight.css.ts';

import { colors } from 'src/styles/colors.css.ts';

globalStyle('html', {
	scrollBehavior: 'smooth',
	height: '100%',
});

globalStyle('body', {
	backgroundColor: colors.verdant[30],
	color: colors.verdant[90],
	height: '100%',
	display: 'grid',
	gridTemplateRows: '4rem minmax(0, 1fr)',
});
