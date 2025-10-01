import time
from playwright.sync_api import sync_playwright

def verify_data_binding():
    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page()
        try:
            print("Navigating to http://localhost:8080...")
            page.goto("http://localhost:8080")

            print("Waiting for page content to load...")
            # Wait for the h1 element to be visible, which indicates the page has loaded.
            page.wait_for_selector("h1:has-text('Two-Way Data Binding Example')", timeout=10000)
            print("Content loaded.")

            # Give it an extra moment for any final rendering
            time.sleep(1)

            screenshot_path = "screenshots/data-binding.png"
            print(f"Taking screenshot: {screenshot_path}")
            page.screenshot(path=screenshot_path)
            print("Screenshot successful.")

        except Exception as e:
            print(f"An error occurred during verification: {e}")
            # In case of an error, take a screenshot for debugging
            page.screenshot(path="screenshots/data-binding_error.png")
            raise  # Re-raise the exception to fail the script
        finally:
            browser.close()

if __name__ == "__main__":
    verify_data_binding()