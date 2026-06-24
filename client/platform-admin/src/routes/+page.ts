// The index route is just an entry point — send visitors to the dashboard.
import { redirect } from '@sveltejs/kit';

export function load(): never {
	throw redirect(307, '/dashboard');
}
