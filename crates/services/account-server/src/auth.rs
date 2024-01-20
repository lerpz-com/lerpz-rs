use rpc::auth_server::Auth;

#[derive(Debug, Default)]
pub struct AuthService {}

#[tonic::async_trait]
impl Auth for AuthService {
	async fn login(
		&self,
		request: tonic::Request<rpc::LoginRequest>,
	) -> Result<tonic::Response<rpc::LoginResponse>, tonic::Status> {
		let response = rpc::LoginResponse {};
	}
}
