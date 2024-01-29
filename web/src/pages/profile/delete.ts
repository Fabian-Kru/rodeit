import { APIContext } from 'astro';

import { rodeit } from 'src/api/mod.ts';
import { getUser, headers } from 'src/session.ts';

export async function POST({ cookies, redirect }: APIContext) {
	const user = getUser(cookies);
	if (!user) {
		return redirect('/login');
	}

	const { response } = await rodeit.DELETE('/users/user/{user_id}', {
		params: {
			path: {
				user_id: Number(user.sub),
			},
		},
		headers: headers(cookies),
	});

	if (!response.ok) {
		console.log(response);
		// TODO: Show error message
		return redirect('/profile');
	}

	cookies.delete('token', {
		path: '/',
	});

	return redirect('/');
}
