import createClient from 'openapi-fetch';

import { paths as Paths } from './types.ts';

export const client = createClient<Paths>({
	baseUrl: import.meta.env.VITE_RODEIT_API_URL,
});

export const picturesBaseUrl = 'https://pictures.captaincoaster.com/1440x1440/';
