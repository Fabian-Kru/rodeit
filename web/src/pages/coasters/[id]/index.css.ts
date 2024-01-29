import { createVar, style } from "@vanilla-extract/css";

import { colors } from "src/styles/colors.css.ts";

export const page = style([{
}])

export const header_image = createVar();

export const header = style([{
	display: 'grid',
	gridTemplateColumns: 'minmax(0, 48rem)',
	rowGap: '1rem',
	justifyContent: 'center',
	alignContent: 'end',
	padding: '2rem',
	aspectRatio: '4',
	backgroundImage: [
		`linear-gradient(to right top, ${colors.verdant[30]}, oklch(0% 0 0 / 0) 50%)`,
		header_image,
	].join(', '),
	backgroundSize: 'cover',
	backgroundPosition: 'center',
	borderBottomColor: colors.verdant[90],
	borderBottomStyle: 'solid',
	borderBottomWidth: 2,
}]);

export const park = style([{
	display: 'grid',
	columnGap: '1rem',
	gridTemplateColumns: 'max-content 1fr',
}]);

export const main = style([{
	display: 'grid',
	gridTemplateColumns: 'minmax(0, 48rem)',
	justifyContent: 'center',
	paddingBlock: '2rem',
}])

export const actions = style([{
	display: 'grid',
	gridAutoFlow: 'column',
	justifyContent: 'start',
	columnGap: '1rem',
}]);
