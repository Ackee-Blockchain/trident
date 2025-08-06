---
hide:
  - toc
  - navigation
---

# Additional Features

Trident offers various additional features to take your fuzzing experience to the next level.

## Code Coverage

Trident's code coverage feature analyzes which parts of your Solana program are tested during fuzzing sessions, helping you identify untested code paths and improve test effectiveness.

<video id="dashboard-video" width="50%" controls autoplay muted loop>
  <source src="../images/codecoverage.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

Key features:

- **Real-time Analysis**: Monitor coverage as fuzzing progresses
- **Multiple Formats**: Generate reports in JSON and HTML formats
- **VS Code Integration**: Visualize coverage directly in your IDE using [Solana VS Code extension](https://marketplace.visualstudio.com/items?itemName=AckeeBlockchain.solana)
- **Detailed Reporting**: Get insights into line coverage, branch coverage, and execution paths

For configuration options, see the [Fuzzing Coverage section](../trident-manifest/index.md#fuzzing-coverage) in the Trident manifest documentation.

## Dashboard

The Trident Dashboard provides a web-based interface for visualizing fuzzing session results through interactive charts and comprehensive statistics.

<video id="dashboard-video" width="50%" controls autoplay muted loop>
  <source src="../images/dashboard.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

Features:

- **Session Statistics**: View execution counts, success rates, and performance metrics
- **Interactive Charts**: Explore results through dynamic visualizations
- **Browser Access**: Open in any web browser without additional software
- **Comprehensive Analysis**: Insights into instruction coverage and account interactions

For configuration details, see the [Fuzzing Metrics section](../trident-manifest/index.md#fuzzing-metrics) in the Trident manifest documentation.

## TridentSVM

TridentSVM is a fast execution and accounts storage environment which is utilized within the Trident.


Github repository: [Trident SVM](https://github.com/Ackee-Blockchain/trident-svm)

TridentSVM uses [Anza's SVM API](https://www.anza.xyz/blog/anzas-new-svm-api)