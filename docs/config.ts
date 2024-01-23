import { OpenAPIV3_1 } from 'openapi-types';
import { Spec } from './util/openapi.ts';

import records_spec from '../services/records/swagger.yml';
import bucket_list_spec from '../services/bucket_list/openapi.json';

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
