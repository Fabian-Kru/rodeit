import { createAsync } from '@solidjs/router';
import { useUser } from 'src/user_session.ts';

export const route = {
	load: () => useUser(),
};

export default function () {
	const user = createAsync(useUser);
	return <h1>{user()?.sub ?? 'none'}</h1>;
}
