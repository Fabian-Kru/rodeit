import { APIContext } from 'astro';

import { rodeit } from 'src/api/mod.ts';

export const FIELD_USERNAME = 'username';
export const FIELD_PASSWORD = 'password';

export async function POST({ request, cookies, redirect }: APIContext) {
	const formData = await request.formData();
	const username = formData.get(FIELD_USERNAME);
	const password = formData.get(FIELD_PASSWORD);
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
	const { data } = await rodeit.POST('/users/login', {
		body: {
			username: username.toString(),
			password: password.toString(),
		},
	});
	if (data === undefined) {
		return redirect('/login?invalid');
	}
	cookies.set('token', data.token, {
		path: '/',
	});
	return redirect('/');
}
