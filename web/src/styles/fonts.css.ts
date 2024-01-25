import { FontMetrics } from '@capsizecss/core';
import { fontFace } from '@vanilla-extract/css';

import { FontRule, createFont } from 'src/styles/internal/font.css.ts';

import { lg, sm, xl } from './screens.css.ts';

const font_face_geist = fontFace({
	src: `url("src/assets/fonts/geist_variable.woff2") format("woff2")`,
	fontWeight: '100 900',
	fontStyle: 'normal',
});

const metrics_geist: FontMetrics = {
	familyName: font_face_geist.slice(1, -1),
	category: 'sans-serif',
	capHeight: 710,
	ascent: 920,
	descent: -220,
	lineGap: 100,
	unitsPerEm: 1000,
	xHeight: 530,
	xWidthAvg: 446,
};

export const geist = createFont(metrics_geist);

export const display_large_rule = geist([
	{
		letterSpacing: -0.5,
		fontOptions: {
			capHeight: 40,
			lineGap: 16,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 40,
			lineGap: 16,
		},
	}),
	xl<FontRule>({
		fontOptions: {
			capHeight: 40,
			lineGap: 16,
		},
	}),
]);

export const display_medium_rule = geist([
	{
		fontOptions: {
			capHeight: 30,
			lineGap: 16,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 32,
			lineGap: 16,
		},
	}),
	lg<FontRule>({
		fontOptions: {
			capHeight: 48,
			lineGap: 24,
		},
	}),
	xl<FontRule>({
		fontOptions: {
			capHeight: 64,
			lineGap: 32,
		},
	}),
]);

export const display_small_rule = geist([
	{
		fontOptions: {
			capHeight: 24,
			lineGap: 16,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 24,
			lineGap: 16,
		},
	}),
	xl<FontRule>({
		fontOptions: {
			capHeight: 24,
			lineGap: 16,
		},
	}),
]);

export const title_large_rule = geist([
	{
		fontWeight: 500,
		fontOptions: {
			capHeight: 18,
			lineGap: 12,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 18,
			lineGap: 12,
		},
	}),
]);

export const title_medium_rule = geist([
	{
		fontWeight: 520,
		fontOptions: {
			capHeight: 15,
			lineGap: 12,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 16,
			lineGap: 12,
		},
	}),
]);

export const title_small_rule = geist([
	{
		fontWeight: 540,
		fontOptions: {
			capHeight: 13,
			lineGap: 12,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 14,
			lineGap: 12,
		},
	}),
]);
export const label_large_rule = geist([
	{
		fontWeight: 600,
		fontFeatureSettings: '"ss09"',
		fontOptions: {
			capHeight: 10,
			lineGap: 10,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 11,
			lineGap: 11,
		},
	}),
]);

export const label_medium_rule = geist([
	{
		fontWeight: 600,
		fontFeatureSettings: '"ss09"',
		fontOptions: {
			capHeight: 9,
			lineGap: 9,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 10,
			lineGap: 10,
		},
	}),
]);

export const label_small_rule = geist([
	{
		fontWeight: 600,
		fontFeatureSettings: '"ss09"',
		fontOptions: {
			capHeight: 8,
			lineGap: 8,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 9,
			lineGap: 9,
		},
	}),
]);

export const body_large_rule = geist([
	{
		fontWeight: 500,
		fontFeatureSettings: '"ss09"',
		fontOptions: {
			capHeight: 11,
			lineGap: 11,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 12,
			lineGap: 12,
		},
	}),
]);

export const body_medium_rule = geist([
	{
		fontWeight: 500,
		fontFeatureSettings: '"ss09"',
		fontOptions: {
			capHeight: 10,
			lineGap: 10,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 11,
			lineGap: 11,
		},
	}),
]);

export const body_small_rule = geist([
	{
		fontWeight: 500,
		fontFeatureSettings: '"ss09"',
		fontOptions: {
			capHeight: 9,
			lineGap: 9,
		},
	},
	sm<FontRule>({
		fontOptions: {
			capHeight: 10,
			lineGap: 10,
		},
	}),
]);
