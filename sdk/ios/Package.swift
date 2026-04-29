// swift-tools-version:5.7
import PackageDescription

let package = Package(
    name: "OminiConnect",
    platforms: [.iOS(.v15), .macOS(.v12)],
    products: [
        .library(
            name: "OminiConnect",
            targets: ["OminiConnect"]
        ),
    ],
    targets: [
        .target(
            name: "OminiConnect",
            dependencies: [],
            path: "Sources/OminiConnect"
        ),
        .testTarget(
            name: "OminiConnectTests",
            dependencies: ["OminiConnect"],
            path: "Tests"
        ),
    ]
)
