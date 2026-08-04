use async_trait::async_trait;
use brazier::*;

// 1. Define your Request payload and associate its response type via generic constraint
#[derive(Debug)]
pub struct CreateUserCommand {
    pub username: String,
}

// In brazier, the Request trait specifies the type it resolves to
impl Request<String> for CreateUserCommand {}

// 2. Create the Handler structure and implement the Async RequestHandler trait
pub struct CreateUserHandler;

#[async_trait]
impl RequestHandler<CreateUserCommand, String> for CreateUserHandler {
    async fn handle(&mut self, request: CreateUserCommand) -> Result<String> {
        // Encapsulated business execution logic
        println!("Database: Storing user '{}' into ledger.", request.username);

        Ok(format!("Successfully created user: {}", request.username))
    }
}

// 3. Setup the entry point via the Tokio runtime
#[tokio::main]
async fn main() -> Result<()> {
    // Construct the central message router
    let mut mediator = Mediator::new();

    // Register our handler to process incoming Commands
    mediator.register_handler(CreateUserHandler);

    // Build the payload
    let command = CreateUserCommand {
        username: "Alice".to_string(),
    };

    // Dispatch via .send() seamlessly linking command to handler
    let execution_result = mediator.send(command).await?;

    println!("Response from Mediator execution: {}", execution_result);

    Ok(())
}
