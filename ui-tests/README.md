# ui-tests

This is a directory containing the Playwright UI tests. It also contains an express server to serve the html generated in the pipeline - this is because GitHub actions would not accept any permutation of a path to the html and I gave up after 50 commits.

To run this, you can use the `PLAYWRIGHT_TEST_BASE_URL` env variable to pass in the path to the `index.html` file and then use playwright (idk, just use IntelliJ).

Will test against Firefox, Chromium, Webkit, Safari-Mobile and Chrome-mobile.