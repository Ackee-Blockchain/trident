# Fuzz Test Example for Trdelnik Fuzzer

---

## Program and Errors Description

- ### Initialize
    - In this function, we initialize a Counter Account, set the count to zero, and assign Authority to the Signer.

- ### Update
    - Based on the Instruction inputs, we update the count variable within the Counter Account. The eligible Update Authority must sign the Transaction.
    - ❗ **NOTE:** 🐛 In the **buggy_math_function** that performs computations to obtain the new count, we fail to properly check input values. This oversight can result in **🚨division by zero🚨** or **🚨subtract with overflow 🚨**panic.


## Fuzz Test Checks
- ✔️ For this example, we do not need any specific checks because the **Fuzzer will automatically panic** when it detects **subtraction with overflow** or **division-by-zero** errors.
