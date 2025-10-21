# Code Coverage

Trident's code coverage feature analyzes which parts of your Solana program are tested during fuzzing sessions, helping you identify untested code paths and improve test effectiveness.

<video id="dashboard-video" width="50%" controls autoplay muted loop>
  <source src="../../images/codecoverage.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

Key features:

- **Real-time Analysis**: Monitor coverage as fuzzing progresses
- **Multiple Formats**: Generate reports in JSON and HTML formats
- **VS Code Integration**: Visualize coverage directly in your IDE using [Solana VS Code extension](https://marketplace.visualstudio.com/items?itemName=AckeeBlockchain.solana)
- **Detailed Reporting**: Get insights into line coverage, branch coverage, and execution paths

## Gathering Coverage Data

1. Enable coverage in the [Trident manifest](../../trident-manifest/index.md#fuzzing-coverage)
2. Deploy your program through the [entrypoint](../../trident-manifest/index.md#entrypoint-deployment)
3. Run the fuzz test

## Viewing Coverage Reports

You have two options for displaying coverage data: generate an HTML report for browser viewing, or generate a JSON report for VS Code integration.

### Live Coverage Updates

For real-time coverage monitoring during fuzzing:

1. Set `format = "json"` in the [Trident manifest](../../trident-manifest/index.md#fuzzing-coverage)
2. Set `loopcount` to a value other than `0` e.g., `5` for frequent updates, for more info check out [Trident manifest](../../trident-manifest/index.md#entrypoint-deployment)
3. Install the [Solana VS Code extension](https://marketplace.visualstudio.com/items?itemName=AckeeBlockchain.solana)
4. Choose one of these options:
    - **Automatic**: Set `attach_extension = true` in the [Trident manifest](../../trident-manifest/index.md#fuzzing-coverage) to automatically show live updates while the fuzz test is running
    - **Manual**: Use the command `Solana: Show Code Coverage` and select the "Attach to active fuzzing session" option

### Post-Session Coverage Analysis

1. Set `format = "json"` in the [Trident manifest](../../trident-manifest/index.md#fuzzing-coverage) before running the fuzz test
2. Run the fuzz test
3. Install the [Solana VS Code extension](https://marketplace.visualstudio.com/items?itemName=AckeeBlockchain.solana)
4. Run the VS Code command `Solana: Show Code Coverage` and select the "Load generated JSON report" option

### HTML Report

1. Set `format = "html"` in the [Trident manifest](../../trident-manifest/index.md#fuzzing-coverage) before running the fuzz test
2. Run the fuzz test
3. After the fuzz test finishes, the HTML report will be stored in the `<fuzz-test-name>-coverage-report` folder
4. Open the HTML report in your browser
