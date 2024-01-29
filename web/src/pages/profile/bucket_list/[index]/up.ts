import { APIContext } from 'astro';

import { rodeit } from 'src/api/mod.ts';
import { getUser, headers } from 'src/session.ts';

export const FIELD_COASTER_IDS = 'coaster_ids';

export async function POST({ params, request, cookies, redirect }: APIContext) {
	const user = getUser(cookies);
	if (!user) {
		return redirect('/login');
	}

	const index = Number(params.index!);
	if (isNaN(index) || index <= 0) {
		return redirect('/profile/bucket_list');
	}

	const formData = await request.formData();
	if (!formData.has(FIELD_COASTER_IDS)) {
		return redirect('/profile/bucket_list');
	}
	const coasterIdsJson = formData.get(FIELD_COASTER_IDS);
	if (!coasterIdsJson) {
		return redirect('/profile/bucket_list');
	}
	const coasterIds: number[] = JSON.parse(coasterIdsJson.toString());
	if (coasterIds.length === 0) {
		return redirect('/profile/bucket_list');
	}
	const [coasterId] = coasterIds.splice(index, 1);
	coasterIds.splice(index - 1, 0, coasterId!);

	await rodeit.PUT('/bucket_list/{user_id}', {
		params: {
			path: {
				user_id: Number(user.sub),
			},
		},
		body: coasterIds,
		headers: headers(cookies),
	});

	return redirect('/profile/bucket_list');
}
