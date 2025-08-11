# Trident Documentation


Fuzzing is a technique to test the robustness of a program by providing it with random inputs.

Trident is a Manually Guided Fuzzing framework for Solana. It lets you to execute instructions of your Solana programs with random data and observe the changes in the state of the program.


##  Getting started

 - [Installation](./basics/installation.md) - Install the Trident Fuzz Testing Framework
 - [Start Fuzzing](./start-fuzzing/index.md) - Focus on security and start fuzzing immediately

## Advanced fuzzing customization

Explore the various features Trident provides.

 - [Transaction Hooks](./trident-advanced/trident-transactions/transaction-hooks/index.md) - Create and manipulate transactions with different methods
 - [Multi-Instruction Transactions](./trident-advanced/trident-transactions/multi-instruction-transactions/index.md) - Compose fuzzed transactions with multiple instructions
 - [Trident Manifest](./trident-manifest/index.md) - Customize your fuzz tests with different configurations
 - [Code Coverage](./trident-advanced/code-coverage/index.md) - Track and display fuzz test code coverage
 - [Dashboard](./trident-advanced/dashboard/index.md) - A web-based interface for visualizing fuzzing session results
 - [Regression Testing](./trident-advanced/regression/index.md) - Track and compare fuzzing session results

## API & Macros Reference

Check out the API and macro reference for Trident.

 - [Trident API & Macros](./trident-api-macro/index.md)

## Trident Examples

Check out examples to learn how to use Trident.

 - [Trident Examples](./trident-examples/trident-examples.md)

## TridentSVM

Check out the TridentSVM, lightweight fuzzing execution and account storage environment.

 - [TridentSVM](./trident-svm/index.md)


## What is Fuzzing ?

*"Fuzz testing is an automated technique that provides generated random, invalid, or unexpected input data to your program. This helps discover unknown bugs and vulnerabilities, potentially preventing zero-day exploits."*

<div id="fuzz-asciinema" style="z-index: 1; position: relative;"></div>
<script>
  window.onload = function(){
    AsciinemaPlayer.create('./images/trident.cast', document.getElementById('fuzz-asciinema'), { preload: true, autoPlay: true, rows: 35 });
}
</script>
