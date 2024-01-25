import { OpenAPIV3_1 } from 'openapi-types';

import pkg from './package.json';
import { Spec } from './util/openapi.ts';

import records_spec from '../services/records/swagger.yml';
import bucket_list_spec from '../services/bucket_list/openapi.json';

export const root: OpenAPIV3_1.Document = {
	openapi: '3.1.0',
	info: {
		title: 'Rodeit API',
		version: pkg.version,
	},
	components: {
		securitySchemes: {
			'jwt': {
				type: 'http',
				scheme: 'bearer',
				bearerFormat: 'JWT',
			}
		}
	}
};


export const specs: Spec[] = [
	{
		name: 'Records',
		route: '/records',
		spec: records_spec as OpenAPIV3_1.Document,
	},
	{
		name: 'Bucket List',
		route: '/bucket_list',
		spec: bucket_list_spec as OpenAPIV3_1.Document,
	},
];
