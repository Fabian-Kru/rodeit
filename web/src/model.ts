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
	id: number;
	rollercoaster_id: number;
	timestamp: string;
	user_id: string;
};
