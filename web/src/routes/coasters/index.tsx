import { Show, createResource } from "solid-js";

import { client } from "src/api/mod.ts";
import { body_large } from "src/styles/atomic/fonts.css";

export default function() {
	// cablcs means coastersAndBucketListCounts
	const [cablcs] = createResource(async () => {
		const { data } = await client.GET("/bucket_list/")
		if (data === undefined) {
			throw new Error("failed to load coasters");
		}
		return data;
	});

	return <>
		<ol>
			{cablcs() && cablcs()!.map((cablc) => {
				return <li>
					<Show when={cablc.coaster.image}>
						{(image) => <img src={`https://pictures.captaincoaster.com/1440x1440/${image()}`} alt={`${cablc.coaster.name} at ${cablc.coaster.park?.name}`} />}
					</Show>
					<p class={body_large}>{cablc.coaster.name}</p>
					<p class={body_large}>{cablc.coaster.park?.name}</p>
					<p class={body_large}>{cablc.coaster.park?.country}</p>
					<p class={body_large}>{cablc.bucket_list_count}</p>
				</li>
			})}
		</ol>
	</>;
}
