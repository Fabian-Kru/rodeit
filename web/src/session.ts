import { AstroCookies } from 'astro';
import { jwtDecode } from 'jwt-decode';

export type User = {
	sub: string;
};

export function getUser(cookies: AstroCookies): User | undefined {
	if (!cookies.has('token')) {
		return undefined;
	}
	const token = cookies.get('token');
	if (!token) {
		return undefined;
	}
	const user = jwtDecode<User>(token.value);
	return user;
}

export function headers(cookies: AstroCookies): Headers | undefined {
	if (!cookies.has('token')) {
		return undefined;
	}
	const token = cookies.get('token');
	if (!token) {
		return undefined;
	}
	const headers = new Headers();
	headers.set('Authorization', `Bearer ${token.value}`);
	return headers;
}
