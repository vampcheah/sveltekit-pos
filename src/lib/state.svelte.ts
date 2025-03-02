import type { UserType } from './types';

export const login_user = $state<UserType>({
	user_id: '',
	nickname: '',
	loginAccessToken: '',
	balances: null
});

export const userLogout = () => {
	login_user.user_id = '';
	login_user.nickname = '';
	login_user.loginAccessToken = '';
	login_user.balances = null;
};
