/// <reference path="../.astro/types.d.ts" />
/// <reference types="astro/client" />

// eslint-disable-next-line @typescript-eslint/consistent-type-definitions
interface ImportMetaEnv {
	readonly VITE_RODEIT_API_URL: string;
	readonly VITE_CAPTAIN_COASTER_API_KEY: string;
}

// eslint-disable-next-line @typescript-eslint/consistent-type-definitions
interface ImportMeta {
	readonly env: ImportMetaEnv;
}
