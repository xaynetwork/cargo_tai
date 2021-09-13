import SwiftUI

struct ContentView: View {
    var body: some View {
        Text("Cargo Tai!")
            .padding()
    }

    func force_rust_bindings() {
        run_cargo_tai_runner();
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
