// @refresh reload
import { RouteSectionProps, Router } from '@solidjs/router';
import { FileRoutes } from '@solidjs/start';
import { Component, Suspense } from 'solid-js';

import 'src/styles/global/mod.css.ts';
import { label_large } from 'src/styles/atomic/fonts.css';

import { nav, nav_ul } from './app.css.ts';

const Root: Component<RouteSectionProps> = function(props) {
	return (
		<>
			<nav class={nav}>
				<ul class={nav_ul}>
					<li class="pages">
						<ul>
							<li>
								<a href="/coasters">
									<p class={label_large}>Coasters</p>
								</a>
							</li>
							<li>
								<a href="/users">
									<p class={label_large}>Users</p>
								</a>
							</li>
						</ul>
					</li>
					<li class="auth">
						<ul>
							<li>
								<a href="/login">
									<p class={label_large}>Sign in</p>
								</a>
							</li>
							<li>
								<a href="/signup">
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
