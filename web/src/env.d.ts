/// <reference path="../.astro/types.d.ts" />
/// <reference types="astro/client" />

// eslint-disable-next-line @typescript-eslint/consistent-type-definitions
interface ImportMetaEnv {
	readonly VITE_RODEIT_API_URL: string;
}

// eslint-disable-next-line @typescript-eslint/consistent-type-definitions
interface ImportMeta {
	readonly env: ImportMetaEnv;
}
