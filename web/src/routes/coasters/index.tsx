import { Show, createResource } from 'solid-js';
import { assignInlineVars } from '@vanilla-extract/dynamic';

import { client, picturesBaseUrl } from 'src/api/mod.ts';
import { body_large, label_large, title_medium } from 'src/styles/atomic/fonts.css.ts';

import {
	aside,
	aside_link,
	coaster,
	coaster_actions,
	coaster_details,
	coaster_details_image,
	coaster_details_meta,
	coaster_details_park,
	coasters,
	main,
	page as page_style,
} from './index.css.ts';
import { useSearchParams } from '@solidjs/router';

type OrderBy = 'rating' | 'ridden' | 'wanted';

export default function () {
	const [searchParams, setSearchParams] = useSearchParams();

	function page(): number {
		if (searchParams.page === undefined || Number(searchParams.page) < 1) {
			return 1;
		}
		return Number(searchParams.page);
	}

	function orderBy(): OrderBy {
		switch (searchParams.orderBy) {
			case 'rating':
			case 'ridden':
			case 'wanted':
				return searchParams.orderBy as OrderBy;
			default:
				// TODO: use redirect instead of fallback
				return 'rating';
		}
	}

	// cablcs means coastersAndBucketListCounts
	const [cablcs] = createResource(orderBy(), async function (source) {
		switch (source) {
			case 'rating': // TODO: implement
			case 'ridden': // TODO: implement
			case 'wanted': {
				const { data } = await client.GET('/bucket_list/');
				if (data === undefined) {
					throw new Error('failed to load coasters');
				}
				return data;
			}
		}
	});

	return (
		<div class={page_style}>
			<aside class={aside}>
				<ul>
					<li>
						<a class={aside_link} href="?orderBy=rating">
							<p class={label_large}>Best Rating</p>
						</a>
					</li>
					<li>
						<a class={aside_link} href="?orderBy=ridden">
							<p class={label_large}>Most Ridden</p>
						</a>
					</li>
					<li>
						<a class={aside_link} href="?orderBy=wanted">
							<p class={label_large}>Most Wanted</p>
						</a>
					</li>
				</ul>
			</aside>
			<main class={main}>
				<ol class={coasters}>
					{cablcs() &&
						cablcs()!.map((cablc) => {
							return (
								<li class={coaster}>
									<div
										class={coaster_details}
										style={assignInlineVars({
											[coaster_details_image]: `url(${picturesBaseUrl}${cablc.coaster.image})`,
										})}
									>
										<p class={`${coaster_details_meta} ${body_large}`}>
											wanted by {cablc.bucket_list_count} user
											{cablc.bucket_list_count != 1 ? 's' : ''}
										</p>
										<div class={coaster_details_park}>
											<Show when={cablc.coaster.park?.country}>
												{(country) => (
													<img
														height={12}
														width={18}
														alt={`${country()} flag`}
														src={`http://purecatamphetamine.github.io/country-flag-icons/3x2/${country()}.svg`}
													/>
												)}
											</Show>
											<p class={body_large}>{cablc.coaster.park?.name}</p>
										</div>
										<p class={title_medium}>{cablc.coaster.name}</p>
									</div>
									<Show when={true}>
										<ul class={coaster_actions}>
											<li></li>
											<li></li>
										</ul>
									</Show>
								</li>
							);
						})}
				</ol>
			</main>
		</div>
	);
}
