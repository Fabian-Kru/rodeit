// @refresh reload
import { RouteSectionProps, Router } from '@solidjs/router';
import { FileRoutes } from '@solidjs/start';
import { Component, Suspense } from 'solid-js';

import 'src/styles/global/mod.css.ts';
import { label_large } from 'src/styles/atomic/fonts.css';

import {
	nav,
	nav_link,
	nav_link_container,
	nav_pages,
	nav_ul,
	nav_user_actions,
} from './app.css.ts';

const Root: Component<RouteSectionProps> = function(props) {
	return (
		<>
			<nav class={nav}>
				<ul class={nav_ul}>
					<li>
						<ul class={nav_pages}>
							<li class={nav_link_container}>
								<a href="/coasters" class={nav_link}>
									<p class={label_large}>Coasters</p>
								</a>
							</li>
							<li class={nav_link_container}>
								<a href="/users" class={nav_link}>
									<p class={label_large}>Users</p>
								</a>
							</li>
						</ul>
					</li>
					<li>
						<ul class={nav_user_actions}>
							<li class={nav_link_container}>
								<a href="/login" class={nav_link}>
									<p class={label_large}>Sign in</p>
								</a>
							</li>
							<li class={nav_link_container}>
								<a href="/signup" class={nav_link}>
									<p class={label_large}>Sign up</p>
								</a>
							</li>
						</ul>
					</li>
				</ul>
			</nav>
			<Suspense>{props.children}</Suspense>
		</>
	);
};

export default function() {
	return (
		<Router root={Root}>
			<FileRoutes />
		</Router>
	);
}
