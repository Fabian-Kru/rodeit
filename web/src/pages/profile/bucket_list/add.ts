import { APIContext } from 'astro';

import { rodeit } from 'src/api/mod.ts';
import { getUser, headers } from 'src/session.ts';

export async function POST({ request, cookies, redirect }: APIContext) {
	const user = getUser(cookies);
	if (!user) {
		return redirect('/login');
	}

	const formData = await request.formData();
	if (!formData.has('coaster_id')) {
		return new Response(null, {
			status: 400,
			statusText: 'coaster_id is required',
		});
	}
	const coaster_id = formData.get('coaster_id');
	if (!coaster_id) {
		return new Response(null, {
			status: 400,
			statusText: 'coaster_id is required',
		});
	}

	const { response } = await rodeit.POST('/bucket_list/{user_id}', {
		params: {
			path: {
				user_id: Number(user.sub),
			},
		},
		headers: headers(cookies),
		body: Number(coaster_id.toString()),
	});

	if (!response.ok) {
		return new Response(null, {
			status: 500,
			statusText: 'failed to add coaster to bucket list',
		});
	}

	return redirect('/profile/bucket_list');
}
