**Target:** Are We Web Yet? (arewewebyet.org)

**Action:** Add Fenrix to the list of web frameworks.

**Instructions:**

To add Fenrix to the "Are We Web Yet?" website, a pull request needs to be made to the `rust-lang/arewewebyet` GitHub repository.

1.  **Navigate to the file:** `content/topics/frameworks.md`
2.  **Edit the file:** In the frontmatter of the file, find the `packages` array.
3.  **Add the crate:** Add `"fenrix"` to the list of packages in the array. The list should be kept in alphabetical order.

**Example of the required change:**

```diff
--- a/content/topics/frameworks.md
+++ b/content/topics/frameworks.md
@@ -X,Y +X,Z @@
 [extra]
 packages = [
   "actix-web",
   "axum",
+  "fenrix",
   "gotham",
   "rocket",
   # ... and so on
 ]
```

This will add Fenrix to the "Web Frameworks" section of the site. No further content is needed, as the site automatically pulls information from crates.io.