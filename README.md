## 1. What is this ?

I create this repository to help you guys to make a better image uploader beside using google storage or s3. If you doing a small to medium project, maybe you need this service in your server to handle image uploader.

## 2. I can upload Image with PHP, Why do i need this service?

Yes you can, but to build a better service and clean code your REST API can be using json only and put multipart/form-data in other service.

## 3. How to Install

1. [Install Rust](https://www.rust-lang.org/learn/get-started) or run
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Run the app using run.sh or run this command in your command line/terminal
```
ROCKET_PORT=4000 cargo watch -x run
```
3. Build using this command (Change your target based on your server platform) or using build.sh
```
cross build --release
```

## 14. License

Licensed under the MIT License, Copyright © 2022 Veirry Augusman. See [LICENSE](https://github.com/VeirryAu/image-upload-with-rust/blob/master/LICENSE) for more information.