use rpc::account_server::Account;

#[derive(Debug, Default)]
pub struct AccountService {}

#[tonic::async_trait]
impl Account for AccountService {
	async fn create_account(
		&self,
		request: tonic::Request<rpc::CreateAccountRequest>,
	) -> Result<tonic::Response<rpc::CreateAccountResponse>, tonic::Status> {
		todo!();
	}

	async fn get_account(
		&self,
		request: tonic::Request<rpc::GetAccountRequest>,
	) -> Result<tonic::Response<rpc::GetAccountResponse>, tonic::Status> {
		todo!();
	}

	async fn validate_credentials(
		&self,
		request: tonic::Request<rpc::ValidateCredentialsRequest>,
	) -> Result<tonic::Response<rpc::ValidateCredentialsResponse>, tonic::Status> {
		todo!();
	}
}
