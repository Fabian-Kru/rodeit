import { APIContext } from 'astro';

import { rodeit } from 'src/api/mod.ts';

export const FIELD_USERNAME = 'username';
export const FIELD_PASSWORD = 'password';
export const FIELD_NAME = 'name';
export const FIELD_SURNAME = 'surname';

export async function POST({ request, redirect }: APIContext) {
	const formData = await request.formData();
	const username = formData.get(FIELD_USERNAME);
	const password = formData.get(FIELD_PASSWORD);
	const name = formData.get(FIELD_NAME);
	const surname = formData.get(FIELD_SURNAME);
	if (!username) {
		return new Response(null, {
			status: 400,
			statusText: 'username is required',
		});
	}
	if (!password) {
		return new Response(null, {
			status: 400,
			statusText: 'password is required',
		});
	}
	if (!name) {
		return new Response(null, {
			status: 400,
			statusText: 'name is required',
		});
	}
	if (!surname) {
		return new Response(null, {
			status: 400,
			statusText: 'surname is required',
		});
	}
	const { data } = await rodeit.POST('/users/register', {
		body: {
			username: username.toString(),
			password: password.toString(),
			name: name.toString(),
			surname: surname.toString(),
		},
	});
	if (data === undefined) {
		return new Response(null, {
			status: 400,
			statusText: 'cannot register with these credentials',
		});
	}
	return redirect('/login?registered');
}
