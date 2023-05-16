<p align="center">
    <img src="https://camo.githubusercontent.com/517483ae0eaba9884f397e9af1c4adc7bbc231575ac66cc54292e00400edcd10/68747470733a2f2f7777772e6d756c7469736166657061792e636f6d2f66696c6561646d696e2f74656d706c6174652f696d672f6d756c7469736166657061792d6c6f676f2d69636f6e2e737667" width="400px" position="center">
</p>

<h1 align="center">
    <b>
        <span style="color: #009fff">Rust Software</span>
        <span style="color: #00005a">Development Kit</span>
    </b>
</h1>

## Introduction to MultiSafePay
MultiSafepay is a Dutch payment services provider, which takes care of contracts, processing transactions, and collecting payment for a range of local and international payment methods. Start selling online today and manage all your transactions in one place!

## Installation
The MultiSafePay Rust SDK makes use of the "Cargo" toolchain included within rust itself, in order to locally build the library. Run the `cargo build --release` command in your terminal.

## Features
Here, you can review a list of features that is currently available within the crate:

|          Feature          | Available |                            Source                             |
| :-----------------------: | :-------: | :-----------------------------------------------------------: |
|     Generating Orders     |    Yes    |      https://docs.multisafepay.com/reference/createorder      |
|      Fetching Orders      |    Yes    |       https://docs.multisafepay.com/reference/getorder        |
|      Updating Orders      |    No     |      https://docs.multisafepay.com/reference/updateorder      |
|     Capturing Payment     |    No     |    https://docs.multisafepay.com/reference/capturepayment     |
| Cancel Authorized Payment |    No     | https://docs.multisafepay.com/reference/cancelauthorizedorder |
|   Put PAD order on hold   |    No     |   https://docs.multisafepay.com/reference/padputorderonhold   |
|  Extend Order Autoexpire  |    No     |  https://docs.multisafepay.com/reference/padextendautoexpire  |
|       List Gateways       |    No     |     https://docs.multisafepay.com/reference/listgateways      |
|        Get Gateway        |    No     |      https://docs.multisafepay.com/reference/getgateway       |
|    List iDEAL issuers     |    No     |   https://docs.multisafepay.com/reference/listidealissuers    |
|   List payment methods    |    No     | https://docs.multisafepay.com/reference/listpaymentmethods-1  |
|    List payment method    |    No     |   https://docs.multisafepay.com/reference/getpaymentmethod    |
|   Challenge chargebacks   |    No     |  https://docs.multisafepay.com/reference/challengechargeback  |
|       Refund order        |    No     |      https://docs.multisafepay.com/reference/refundorder      |

## Testing The Application
The tests within this project can be ran with the `cargo test` command. This project has built-in tests for all classes associated with the libary.

## Note For Developers
As of **May 16, 2023**, this repository is currently undergoing significant development. Please anticipate substantial API changes in the future.
