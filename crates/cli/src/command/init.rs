use trdelnik_client::TestGenerator;

pub async fn init() {
    let generator = TestGenerator::new();
    generator.generate().await;
}