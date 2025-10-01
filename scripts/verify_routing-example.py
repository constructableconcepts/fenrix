import time
from playwright.sync_api import sync_playwright

def verify_routing_example():
    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page()
        try:
            # --- Verify Home Page ---
            print("Navigating to http://localhost:8080...")
            page.goto("http://localhost:8080")

            print("Waiting for Home page content to load...")
            page.wait_for_selector("h1:has-text('Fenrix Router Example')", timeout=10000)
            page.wait_for_selector("div:has-text('Welcome to the Home Page!')", timeout=5000)
            print("Home page loaded.")

            time.sleep(1)
            home_screenshot_path = "screenshots/routing-example_home.png"
            print(f"Taking screenshot of Home page: {home_screenshot_path}")
            page.screenshot(path=home_screenshot_path)
            print("Home page screenshot successful.")

            # --- Verify About Page Navigation ---
            print("Clicking 'About' link to navigate...")
            page.click("a[href='#/about']")

            print("Waiting for About page content to load...")
            page.wait_for_selector("div:has-text('This is the About Page.')", timeout=5000)
            print("About page loaded.")

            time.sleep(1)
            about_screenshot_path = "screenshots/routing-example_about.png"
            print(f"Taking screenshot of About page: {about_screenshot_path}")
            page.screenshot(path=about_screenshot_path)
            print("About page screenshot successful.")

        except Exception as e:
            print(f"An error occurred during verification: {e}")
            page.screenshot(path="screenshots/routing-example_error.png")
            raise
        finally:
            browser.close()

if __name__ == "__main__":
    verify_routing_example()