import { AstroGlobal } from 'astro';
import { jwtDecode } from 'jwt-decode';

export type User = {
	sub: string;
};

export function getUser(astro: AstroGlobal): User | undefined {
	if (!astro.cookies.has('token')) {
		return undefined;
	}
	const token = astro.cookies.get('token');
	if (!token) {
		return undefined;
	}
	const user = jwtDecode<User>(token.value);
	return user;
}
