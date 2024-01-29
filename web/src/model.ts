export type Coaster = {
	id: number;
	count?: number;
	name?: string;
	image?: string;
	park?: {
		name?: string;
		country?: string;
	};
};

export type Record = {
	coaster?: Coaster;
	date: Date;
};
