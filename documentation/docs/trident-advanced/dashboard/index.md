# Dashboard

The Trident Dashboard provides a web-based interface for visualizing fuzzing session results through interactive charts and comprehensive statistics.

<video id="dashboard-video" width="50%" controls autoplay muted loop>
  <source src="../../images/dashboard.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

Features:

- **Session Statistics**: View execution counts, success rates, and performance metrics
- **Interactive Charts**: Explore results through dynamic visualizations
- **Browser Access**: Open in any web browser without additional software
- **Comprehensive Analysis**: Insights into instruction coverage and account interactions

## Viewing the Dashboard

1. Enable the dashboard in the [Trident manifest](../../trident-manifest/index.md#fuzzing-metrics) by setting `dashboard = true`
2. Run the fuzz test to generate dashboard data
3. Start the dashboard server:
   ```bash
   trident server
   ```
4. Open your browser and navigate to the provided URL (typically `http://localhost:8080`)
