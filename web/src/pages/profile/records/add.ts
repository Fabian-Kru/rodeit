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

	const { response } = await rodeit.POST('/records/record', {
		headers: headers(cookies),
		body: {
			rollercoaster_id: Number(coaster_id.toString()),
			user_id: Number(user.sub),
		},
	});

	if (!response.ok) {
		return new Response(null, {
			status: 500,
			statusText: 'failed to add coaster to records',
		});
	}

	return redirect('/profile/records');
}
