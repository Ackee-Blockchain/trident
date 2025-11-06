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

1. Enable the dashboard in the [Trident manifest](../../trident-manifest/index.md#fuzzing-metrics) by setting `dashboard = true`:

    ```toml
    [fuzz.metrics]
    enabled = true
    dashboard = true
    ```

2. Open another terminal window and execute `trident server` from the fuzz-tests directory.

3. Open your browser and navigate to the provided URL (typically `http://localhost:8080`).

4. Run the fuzz test to generate dashboard data.



## Monitoring Custom Metrics

You can monitor custom fuzzing metrics by adding `trident.add_histogram_metric` to your flow methods:

```rust
#[flow_executor]
impl FuzzTest {
    #[flow]
    fn example_flow(&mut self) {
        let random_value = self.trident.random_from_range(1..1000);
        
        let instruction = create_instruction(random_value);
        let result = self.trident.process_transaction(&[instruction], "example");
        
        // Track the random value in dashboard metrics
        self.trident.add_histogram_metric("random_values", random_value as f64);
    }
}
```