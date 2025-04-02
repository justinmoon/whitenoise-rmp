use anyhow::Result;
use appium_client::ClientBuilder;
use appium_client::capabilities::android::AndroidCapabilities;
use appium_client::capabilities::{AppCapable, AppiumCapability};
use appium_client::find::{AppiumFind, By};
use appium_client::wait::AppiumWait;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Configure capabilities for the Android app
    let mut capabilities = AndroidCapabilities::new();
    capabilities.app("/Users/justin/code/bar/android/app/build/outputs/apk/debug/app-debug.apk");

    // Set required capabilities for Appium
    capabilities.set_str("appium:automationName", "uiautomator2");
    capabilities.set_str("appium:platformName", "Android");
    capabilities.set_str("appium:deviceName", "Android Emulator");

    println!("Connecting to Appium server...");
    let client = ClientBuilder::native(capabilities)
        .connect("http://localhost:4723/")
        .await?;
    println!("Connected to Appium server successfully!");

    // Wait for app to load and initialize
    println!("Waiting for app to fully initialize...");
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Test: check initial counter == 0
    println!("Checking initial counter value...");
    // Try to find with content-description first
    let counter_display = client
        .appium_wait()
        .at_most(Duration::from_secs(30))
        .for_element(By::accessibility_id("counterValue"))
        .await?;

    let initial_text = counter_display.text().await?;
    assert_eq!(initial_text, "0", "Initial counter value should be 0");
    println!("Initial counter value is 0, as expected.");

    // Hit the increment button
    println!("Clicking the increment button...");
    let increment_button = client
        .appium_wait()
        .for_element(By::accessibility_id("incrementButton"))
        .await?;

    increment_button.click().await?;

    // Give the UI a moment to update
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Verify counter == 1
    println!("Checking counter value after increment...");
    let counter_after_increment = client.find_by(By::accessibility_id("counterValue")).await?;

    let after_increment = counter_after_increment.text().await?;
    assert_eq!(after_increment, "1", "Counter should be 1 after increment");
    println!("Counter value is 1 after increment, as expected.");

    // Hit the decrement button
    println!("Clicking the decrement button...");
    let decrement_button = client
        .find_by(By::accessibility_id("decrementButton"))
        .await?;

    decrement_button.click().await?;

    // Give the UI a moment to update
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Verify counter == 0
    println!("Checking counter value after decrement...");
    let counter_after_decrement = client.find_by(By::accessibility_id("counterValue")).await?;

    let after_decrement = counter_after_decrement.text().await?;
    assert_eq!(after_decrement, "0", "Counter should be 0 after decrement");
    println!("Counter value is 0 after decrement, as expected.");

    println!("Test completed successfully!");

    Ok(())
}
