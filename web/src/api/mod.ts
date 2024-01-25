import createClient from 'openapi-fetch';

import { paths as Paths } from './types.ts';

export const client = createClient<Paths>({
	baseUrl: 'http://0.0.0.0:3000/',
});
