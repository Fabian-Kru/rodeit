import { APIContext } from 'astro';

import { rodeit } from 'src/api/mod.ts';
import { getUser, headers } from 'src/session.ts';

export async function POST({ params, cookies, redirect }: APIContext) {
	const user = getUser(cookies);
	if (!user) {
		return redirect('/login');
	}

	await rodeit.DELETE('/bucket_list/{user_id}/{index}', {
		params: {
			path: {
				user_id: Number(user.sub),
				index: params.index,
			},
		},
		headers: headers(cookies),
	});

	return redirect('/profile/bucket_list');
}
