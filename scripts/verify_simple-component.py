import time
from playwright.sync_api import sync_playwright

def verify_simple_component():
    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page()
        try:
            print("Navigating to http://localhost:8080...")
            page.goto("http://localhost:8080")

            print("Waiting for page content to load...")
            page.wait_for_selector("h1:has-text('Simple Component')", timeout=10000)
            print("Content loaded.")

            time.sleep(1)

            screenshot_path = "screenshots/simple-component.png"
            print(f"Taking screenshot: {screenshot_path}")
            page.screenshot(path=screenshot_path)
            print("Screenshot successful.")

        except Exception as e:
            print(f"An error occurred during verification: {e}")
            page.screenshot(path="screenshots/simple-component_error.png")
            raise
        finally:
            browser.close()

if __name__ == "__main__":
    verify_simple_component()