import { OpenAPIV3_1 } from 'openapi-types';
import { merge } from 'ts-deepmerge';

export type Spec = {
	name: string;
	route: string;
	spec: OpenAPIV3_1.Document;
};

const OPERATIONS = ['get', 'put', 'post', 'delete', 'options', 'head', 'patch', 'trace'] as const;

export function merge_specs(specs: Spec[]): OpenAPIV3_1.Document {
	let rebased_specs = specs.map(({ name, route, spec }) => ({
		...spec,
		paths: Object.fromEntries(
			Object.entries(spec.paths!).map(([path, item]) => {
				OPERATIONS.forEach((op) => {
					const operation = item![op];
					if (!operation) return;
					operation.tags ??= [];
					operation.tags.push(name);
				});
				return [`${route}${path}`, item];
			})
		),
	} as OpenAPIV3_1.Document));
	return merge(...rebased_specs) as OpenAPIV3_1.Document;
}
