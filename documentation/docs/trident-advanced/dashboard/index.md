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

1. Open another terminal window and execute `trident server` from the fuzz-tests directory.

2. Open your browser and navigate to the provided URL (typically `http://localhost:8080`).

3. Enable the dashboard in the [Trident manifest](../../trident-manifest/index.md#fuzzing-metrics) by setting `dashboard = true`.
    ```toml
    [fuzz.metrics]
    enabled = true
    dashboard = true
    ```

4. Run the fuzz test to generate dashboard data.



## Monitoring custom fuzzing metrics

Trident allows you to monitor custom fuzzing metrics. If your program expects integer instruction inputs, it is possible to collect data on the randomly generated values and display them in the dashboard.

To do this, you need to add `trident.add_histogram_metric` to the `#[init]`, `#[flow]` or `#[end]` functions. The following code snippet shows how the `InitializeFnTransaction` instruction input is monitored; the statistics will be displayed in the dashboard.


```rust
#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: FuzzAccounts::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        let mut ix = InitializeFnTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident.execute_transaction(&mut ix, Some("Init"));
    }

    #[flow(weight = 5)]
    fn flow1(&mut self) {
        let mut ix = InitializeFnTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut ix, Some("Flow1"));

        // Add the histogram metric to the dashboard
        self.trident
            .add_histogram_metric("flow1_metric", ix.instruction.data.input as f64);
    }

    #[flow(weight = 5)]
    fn flow2(&mut self) { 
      //.... 
    }
    #[flow(weight = 90)]
    fn flow3(&mut self) {
      //....
    } 
    #[end]
    fn cleanup(&mut self) -> Result<(), FuzzingError> {
      //....  
    }
}
```