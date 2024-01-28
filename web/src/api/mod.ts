import createClient from 'openapi-fetch';

import { paths as RodeitPaths } from './rodeit_types.ts';
import { paths as CaptainCoasterPaths } from './captain_coaster_types.ts';

export const rodeit = createClient<RodeitPaths>({
	baseUrl: import.meta.env.VITE_RODEIT_API_URL,
});

const captainCoasterHeaders = new Headers();
captainCoasterHeaders.set(
	'Authorization',
	`Bearer ${import.meta.env.VITE_CAPTAIN_COASTER_API_KEY}`
);
captainCoasterHeaders.set('Accept', 'application/ld+json');

export const captainCoaster = createClient<CaptainCoasterPaths>({
	baseUrl: 'https://captaincoaster.com/',
	headers: captainCoasterHeaders,
});

export const picturesBaseUrl = 'https://pictures.captaincoaster.com/1440x1440/';
