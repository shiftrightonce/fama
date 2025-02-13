#![allow(dead_code)]

use fama::PipelineTrait;

#[tokio::main]
async fn main() {
    // 1. Create an instance of a "CleanProductSKU"
    //    CleanProductSKU implements "PipelineTrait"
    let cleaner = CleanProductSKU;

    // 2. The followings are the methods "PipelineTrait" expose

    // - call "deliver"
    let sku = ProductSKU("123 435 7565".to_string());
    println!("deliver: {:?}", cleaner.deliver(sku).await);

    // - call "try_to_deliver"
    let sku = ProductSKU("123 435 7565".to_string());
    println!("try deliver as: {:?}", cleaner.try_to_deliver(sku).await);

    // - call "confirm"
    let sku = ProductSKU("123 435 7565".to_string());
    println!("confirm : {:?}", cleaner.confirm(sku).await);

    // - call "deliver_as"
    let sku = ProductSKU("123 435 7565".to_string());
    println!("deliver as : {:?}", cleaner.deliver_as::<i32>(sku).await);

    // - call "try_deliver_as"
    let sku = ProductSKU("123 435 7565".to_string());
    println!(
        "try deliver as : {:?}",
        cleaner.try_deliver_as::<i32>(sku).await
    );
}

// 3. The pipeline content
#[derive(Debug, Clone)]
struct ProductSKU(String);

// 4. As usual, we implement the injectable trait
#[fama::async_trait]
impl busybody::Injectable for ProductSKU {
    async fn inject(container: &fama::busybody::ServiceContainer) -> Self {
        container.get_type().await.unwrap()
    }
}

// 5. A struct that will contain on the "pipes"
//    we require to complete the task
struct CleanProductSKU;

// 6. Below is an example of implementing the "fama::PipelineTrait" for a type
//    The trait requires you to implement the "handle_pipe" method
#[fama::async_trait]
impl fama::PipelineTrait for CleanProductSKU {
    type Content = ProductSKU;

    async fn handle_pipe(
        &self,
        pipeline: fama::Pipeline<Self::Content>,
    ) -> fama::Pipeline<Self::Content> {
        pipeline
            // - replace spaces with a dash and store the result
            .store_fn(|mut content: ProductSKU| async move {
                content.0 = content.0.replace(' ', "-");
                content
            })
            .await
            // - store an i32 in the pipe
            .store_fn(|| async { 44 })
            .await
    }
}
