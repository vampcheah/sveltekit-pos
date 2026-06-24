// Open-state for the single logout confirmation dialog, rendered once in the
// (app) layout. Logout triggers (sidebar, header, settings) set `open = true`
// instead of calling `auth.logout()` directly, so the confirm flow is shared
// and the dialog lives in exactly one place.
class LogoutDialog {
	open = $state(false);
}

export const logoutDialog = new LogoutDialog();
