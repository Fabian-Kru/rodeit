'use server';
import { useSession } from '@solidjs/start/server';
import { getRequestEvent } from 'solid-js/web';
import { jwtDecode } from 'jwt-decode';
import { cache } from '@solidjs/router';

export type UserSession = {
	token?: string | undefined;
};

export type User = {
	sub: string;
};

export function useUserSession() {
	return useSession<UserSession>(getRequestEvent()!, {
		// TODO: use env var
		password: 'extremelysuperobviousverysecretohmygawd',
	});
}

export const useUser = cache(async function () {
	const { data } = await useUserSession();
	if (!data.token) {
		return undefined;
	}
	const user = jwtDecode<User>(data.token);
	return user;
}, 'user');
