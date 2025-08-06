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

For configuration options, see the [Fuzzing Coverage section](../../trident-manifest/index.md#fuzzing-coverage) in the Trident manifest documentation.