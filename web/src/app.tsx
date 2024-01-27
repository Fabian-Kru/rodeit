// @refresh reload
import { RouteSectionProps, Router, action, createAsync, redirect } from '@solidjs/router';
import { FileRoutes } from '@solidjs/start';
import { Component, Show, Suspense } from 'solid-js';

import 'src/styles/global/mod.css.ts';
import { label_large } from 'src/styles/atomic/fonts.css.ts';
import { link } from 'src/styles/forms.css.ts';
import { useUser, useUserSession } from 'src/user_session.ts';

import { nav, nav_link_container, nav_pages, nav_ul, nav_user_actions } from './app.css.ts';

const logout = action(async function () {
	'use server';
	const session = await useUserSession();
	await session.update((userSession) => (userSession.token = undefined));
	throw redirect('/', { revalidate: useUser.key });
});

const Root: Component<RouteSectionProps> = function (props) {
	const user = createAsync(useUser);
	return (
		<Suspense>
			<nav class={nav}>
				<ul class={nav_ul}>
					<li>
						<ul class={nav_pages}>
							<li class={nav_link_container}>
								<a href="/coasters" class={link}>
									<p class={label_large}>Coasters</p>
								</a>
							</li>
							<li class={nav_link_container}>
								<a href="/users" class={link}>
									<p class={label_large}>Users</p>
								</a>
							</li>
						</ul>
					</li>
					<li>
						<Show
							when={user()}
							fallback={
								<ul class={nav_user_actions}>
									<li class={nav_link_container}>
										<a href="/login" class={link}>
											<p class={label_large}>Sign in</p>
										</a>
									</li>
									<li class={nav_link_container}>
										<a href="/signup" class={link}>
											<p class={label_large}>Sign up</p>
										</a>
									</li>
								</ul>
							}
						>
							{(user) => (
								<ul class={nav_user_actions}>
									<li class={nav_link_container}>
										<a href="/profile" class={link}>
											<p class={label_large}>{user().sub}</p>
										</a>
									</li>
									<li class={nav_link_container}>
										<form action={logout} method="post">
											<button type="submit" class={link}>
												<p class={label_large}>Sign out</p>
											</button>
										</form>
									</li>
								</ul>
							)}
						</Show>
					</li>
				</ul>
			</nav>
			{props.children}
		</Suspense>
	);
};

export default function () {
	return (
		<Router root={Root}>
			<FileRoutes />
		</Router>
	);
}
