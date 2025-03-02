export const copyToClipboard = async (text: string): Promise<boolean> => {
	try {
		if (navigator.clipboard && window.isSecureContext) {
			await navigator.clipboard.writeText(text);
			return true;
		} else if (window.isSecureContext) {
			// For browsers that support writeText but need user activation
			const permissionResult = await navigator.permissions.query({
				name: 'clipboard-write' as PermissionName
			});
			
			if (permissionResult.state === 'granted' || permissionResult.state === 'prompt') {
				await navigator.clipboard.writeText(text);
				return true;
			}
		}
		
		// If we can't use the clipboard API, inform the user
		console.warn('Clipboard API not available. Please use the device\'s native copy function.');
		return false;
	} catch (err) {
		console.error('Copy failed:', err);
		return false;
	}
};

export const getClipboardContent = async (): Promise<string> => {
	try {
		// Check for Clipboard API support
		if (navigator.clipboard && window.isSecureContext) {
			try {
				const text = await navigator.clipboard.readText();
				return text;
			} catch (err) {
				if (err instanceof Error) {
					// Handle permission denied
					if (err.name === 'NotAllowedError') {
						console.warn('Clipboard permission denied');
					} else {
						console.error('Clipboard read failed:', err);
					}
				}
			}
		}

		// If Clipboard API is not available
		console.warn('Clipboard API not available. Please use the device\'s native paste function.');
		return '';
	} catch (err) {
		console.error('Failed to get clipboard content:', err);
		return '';
	}
};

export const createCopyAction = () => {
	return async (text: string) => {
		return await copyToClipboard(text);
	};
};
