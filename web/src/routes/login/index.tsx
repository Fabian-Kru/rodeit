import { btn, input_text, link } from 'src/styles/forms.css.ts';

import { actions, form, page } from './index.css.ts';
import { label_large } from 'src/styles/atomic/fonts.css.ts';
import { action, redirect } from '@solidjs/router';
import { client } from 'src/api/mod.ts';
import { UserSession, useUser, useUserSession } from 'src/user_session.ts';

const FIELD_USERNAME = 'username';
const FIELD_PASSWORD = 'password';

const login = action(async function (formData: FormData) {
	'use server';
	const username = formData.get(FIELD_USERNAME);
	const password = formData.get(FIELD_PASSWORD);
	if (!username) {
		return new Error('username is required');
	}
	if (!password) {
		return new Error('password is required');
	}
	const { data } = await client.POST('/users/login', {
		body: {
			username: username.toString(),
			password: password.toString(),
		},
	});
	if (data === undefined) {
		throw new Error('invalid credentials');
	}
	try {
		const session = await useUserSession();
		await session.update((userSession: UserSession) => (userSession.token = data.token));
	} catch (err) {
		console.log(err);
		return err as Error;
	}
	throw redirect('/', { revalidate: useUser.key });
});

export default function () {
	return (
		<div class={page}>
			<form class={form} action={login} method="post">
				<input
					class={input_text}
					type="text"
					placeholder="Username"
					name={FIELD_USERNAME}
					required
				/>
				<input
					class={input_text}
					type="text"
					placeholder="Password"
					name={FIELD_PASSWORD}
					required
				/>
				<div class={actions}>
					<a href="/signup" class={link}>
						<div class={label_large}>Sign up</div>
					</a>
					<button type="submit" class={btn}>
						Sign in
					</button>
				</div>
			</form>
		</div>
	);
}
