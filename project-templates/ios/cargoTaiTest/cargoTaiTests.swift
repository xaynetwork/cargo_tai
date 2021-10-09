import XCTest
@testable import cargoTai

class cargoTaiTests: XCTestCase {
    func testRustTestSuite() throws {
        let result = run_cargo_tai_runner()
        XCTAssert(result == 0)
    }
}
