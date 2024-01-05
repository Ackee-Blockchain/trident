# Fuzz Test Example for Trdelnik Fuzzer

---

## Program and Errors Description

- ### Initialize
    - Within this function, we initialize Counter Account, set count to zero and also assign Authority to the Signer.
- ### Update
    - Based on the Instruction inputs we update the count variable within the Counter Account. The eligible Update Authority has to sign the Transaction.
    - ❗ **NOTE:** 🐛 Within the **buggy_math_function** that performs computations in order to obtain new count, we do not correctly check input values so that the computation can result in **🚨div-by-zero🚨** or **🚨subtract with overflow panic.🚨**


## Fuzz Test Checks
- ✔️ For this example we do not need any checks because the **Fuzzer will panic automatically** when it notices **subtract with overflow** or **div-by-zero**.
