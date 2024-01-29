import { APIContext } from 'astro';

import { rodeit } from 'src/api/mod.ts';
import { getUser, headers } from 'src/session.ts';

export const FIELD_USERNAME = 'username';
export const FIELD_PASSWORD = 'password';
export const FIELD_NAME = 'name';
export const FIELD_SURNAME = 'surname';

export async function POST({ cookies, request, redirect }: APIContext) {
	const user = getUser(cookies);
	if (!user) {
		return redirect('/login');
	}

	const formData = await request.formData();

	const username = formData.get(FIELD_USERNAME);
	const password = formData.get(FIELD_PASSWORD);
	const name = formData.get(FIELD_NAME);
	const surname = formData.get(FIELD_SURNAME);

	await rodeit.PATCH('/users/user/{user_id}', {
		params: {
			path: {
				user_id: Number(user.sub),
			},
		},
		headers: headers(cookies),
		body: {
			username: username?.toString(),
			password: password?.toString(),
			name: name?.toString(),
			surname: surname?.toString(),
		},
	});
	return redirect('/profile');
}
