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

MultiSafepay is a prominent Dutch payment services provider renowned for its comprehensive solutions. As an established organization, MultiSafepay facilitates the management of contracts, seamless transaction processing, and efficient collection of payments across a diverse range of local and international payment methods.

By leveraging the services offered by MultiSafepay, businesses can establish a robust online presence and commence selling their products or services with confidence. With MultiSafepay's integrated platform, all transactions can be conveniently monitored and managed within a centralized location, streamlining the payment process and ensuring a seamless customer experience.

Embark on your online sales journey today with MultiSafepay and experience the convenience and efficiency of consolidating your transactions under the umbrella of a trusted payment services provider.

## Installation

The MultiSafePay Rust SDK leverages the built-in "Cargo" toolchain within the Rust programming language to facilitate local library building. To build the library on your local machine, simply execute the cargo build --release command in your terminal.

## Features

Below, you will find an overview of the features currently available within the crate, as well as the features currently being worked on.

|                                          @                                           | Available |
| :----------------------------------------------------------------------------------------: | :-------: |
|          [Generating Orders](https://docs.multisafepay.com/reference/createorder)          |     ✔️     |
|            [Fetching Orders](https://docs.multisafepay.com/reference/getorder)             |     ✔️     |
|           [Updating Orders](https://docs.multisafepay.com/reference/updateorder)           |     🔨     |
|        [Capturing Payment](https://docs.multisafepay.com/reference/capturepayment)         |     ❌     |
| [Cancel Authorized Payment](https://docs.multisafepay.com/reference/cancelauthorizedorder) |     ❌     |
|     [Put PAD order on hold](https://docs.multisafepay.com/reference/padputorderonhold)     |     ❌     |
|   [Extend Order Autoexpire](https://docs.multisafepay.com/reference/padextendautoexpire)   |     ❌     |
|           [List Gateways](https://docs.multisafepay.com/reference/listgateways)            |     🔨     |
|             [Get Gateway](https://docs.multisafepay.com/reference/getgateway)              |     🔨     |
|       [List iDEAL issuers](https://docs.multisafepay.com/reference/listidealissuers)       |     🔨     |
|    [List payment methods](https://docs.multisafepay.com/reference/listpaymentmethods-1)    |     🔨     |
|      [List payment method](https://docs.multisafepay.com/reference/getpaymentmethod)       |     🔨     |
|    [Challenge chargebacks](https://docs.multisafepay.com/reference/challengechargeback)    |     ❌     |
|            [Refund order](https://docs.multisafepay.com/reference/refundorder)             |     ❌     |

## Testing The Application

To run the tests in this project, use the `cargo test` command. We've got built-in tests for all the classes related to the library, so you can make sure everything is working smoothly.

When you run `cargo test` , it checks the behavior of each class, making sure they meet the expected specifications. I take testing seriously to ensure the project is solid and reliable, giving you a hassle-free experience when using the library's features.

### Encountering Errors
If any test fails. Feel free to open an issue on the [GitHub Repository](https://github.com/RikkertTheDeveloper/multisafepay-rust-sdk), and we will take a look at the issue as soon as we can.

## Contributing
We greatly appreciate and encourage contributions to the project. If you would like to contribute, please feel free to open an issue or submit a pull request addressing any problems or improvements. We welcome and value contributions from the community. Your involvement helps us enhance the project and make it even better.

### Contributors
Below is a list of all the people that helped to make the MultiSafePay SDK possible:
- [Rick Arendsen](https://github.com/rikkertthedeveloper)
