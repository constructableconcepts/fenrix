import time
from playwright.sync_api import sync_playwright

def verify_server_function_example():
    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page()
        try:
            # --- Verify Initial State ---
            print("Navigating to http://localhost:8080...")
            page.goto("http://localhost:8080")

            print("Waiting for page content to load...")
            page.wait_for_selector("h1:has-text('Server Function Example')", timeout=10000)
            page.wait_for_selector("div:has-text('No user fetched yet.')", timeout=5000)
            print("Initial content loaded.")

            time.sleep(1)
            initial_screenshot_path = "screenshots/server-function-example_initial.png"
            print(f"Taking screenshot of initial state: {initial_screenshot_path}")
            page.screenshot(path=initial_screenshot_path)
            print("Initial screenshot successful.")

            # --- Trigger Server Function and Verify Update ---
            print("Clicking 'Fetch User from Server' button...")
            page.click("button:has-text('Fetch User from Server')")

            print("Waiting for UI to update with fetched data...")
            page.wait_for_selector("div:has-text('Fetched User: GV (ID: 1)')", timeout=5000)
            print("UI updated successfully.")

            time.sleep(1)
            final_screenshot_path = "screenshots/server-function-example_final.png"
            print(f"Taking screenshot of final state: {final_screenshot_path}")
            page.screenshot(path=final_screenshot_path)
            print("Final screenshot successful.")

        except Exception as e:
            print(f"An error occurred during verification: {e}")
            page.screenshot(path="screenshots/server-function-example_error.png")
            raise
        finally:
            browser.close()

if __name__ == "__main__":
    verify_server_function_example()