export interface BalanceType {
	currency: string;
	amount: number;
	rate: number;
}

export interface UserType {
	user_id: string;
	nickname: string;
	loginAccessToken: string;
	balances: BalanceType[] | null;
}
