# Trident Documentation

##  Getting started

 - [Installation](./basics/installation.md) - Install the Trident Fuzz Testing Framework
 - [Start Fuzzing](./start-fuzzing/start-fuzzing.md) - Focus on security and start fuzzing immediately

## Advanced fuzzing customization

Check out the different features Trident provides.

 - [Transaction Methods](./trident-advanced/trident-transactions/transaction-methods/index.md) - Create and manipulate transactions with different methods
 - [Fuzzing Flows](./trident-advanced/trident-transactions/trident-fuzzing-flows/index.md) - Explore different approaches to fuzz your program
 - [Multi-Instruction Transactions](./trident-advanced/trident-transactions/multi-instruction-transactions/index.md) - Compose fuzzed transactions with multiple instructions
 - [Trident Manifest](./trident-manifest/trident-manifest.md) - Customize your fuzz tests with different configurations

## API & Macros Reference

Check out the API and macro reference for Trident.

 - [Trident API & Macros](./trident-api-macro/index.md) - Reference for Trident's API and macros

## Trident Examples

Check the different examples of Trident.

 - [Trident Examples](./trident-examples/trident-examples.md) - Learn about the different examples of Trident

## TridentSVM

Check out the TridentSVM, lightweight fuzzing execution and account storage environment.

 - [TridentSVM](./trident-svm/trident-svm.md) - Learn about the different examples of Trident


## What is Fuzzing ?

*"Fuzz testing is an automated technique that provides generated random, invalid, or unexpected input data to your program. This helps discover unknown bugs and vulnerabilities, potentially preventing zero-day exploits."*

<div id="fuzz-asciinema" style="z-index: 1; position: relative;"></div>
<script>
  window.onload = function(){
    AsciinemaPlayer.create('./images/trident.cast', document.getElementById('fuzz-asciinema'), { preload: true, autoPlay: true, rows: 35 });
}
</script>
