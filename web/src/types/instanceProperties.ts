export interface InstanceProperties {
	demo: boolean;
	donation: DonationMethod[] | undefined;
}

export interface DonationMethod {
	type: 'Paypal';
	data: string;
}
