import { APIContext } from 'astro';

export async function POST({ cookies, redirect }: APIContext) {
	cookies.delete('token', {
		path: '/',
	});
	return redirect('/');
}
