import { StyleRule, style } from '@vanilla-extract/css';

import { lg, xl } from 'src/styles/screens.css.ts';
import { layout } from 'src/styles/internal/layers.css.ts';

function layoutStyle(rule: StyleRule | StyleRule[], debugId?: string): string {
	if (!Array.isArray(rule)) {
		return style({ '@layer': { [layout]: rule } }, debugId);
	}

	return style(
		rule.map((r) => ({ '@layer': { [layout]: r } })),
		debugId
	);
}

export const grid = layoutStyle(
	[
		{
			display: 'grid',
			gap: '1rem',
		},
		lg({}),
		xl({}),
	],
	'grid'
);
