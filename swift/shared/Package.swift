// swift-tools-version: 5.8

import PackageDescription

let package = Package(
    name: "khiin-swift-shared",
    platforms: [
        .macOS("13.0"),
        .iOS("15.0")
    ],
    products: [
        .library(
            name: "KhiinSwift",
            targets: ["KhiinSwift"]
        )
    ],
    dependencies: [
        .package(
            url: "https://github.com/apple/swift-protobuf.git",
            .upToNextMajor(from: "1.21.0")
        ),
        .package(
            url: "https://github.com/SwiftyBeaver/SwiftyBeaver.git",
            .upToNextMajor(from: "2.0.0")
        ),
        .package(
            path: "../KhiinBridge"
        )
    ],
    targets: [
        .target(
            name: "KhiinSwift",
            dependencies: [
                "KhiinBridge",
                "SwiftyBeaver",
                .product(
                    name: "SwiftProtobuf",
                    package: "swift-protobuf"
                )
            ],
            path: ".",
            sources: ["src"]
        )
    ]
)
